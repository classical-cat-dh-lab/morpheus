#!/usr/bin/env python3
"""Compile preserved Perseus XML to independently licensed offline dictionary packs."""
import gzip
import hashlib
import html
import io
import json
from pathlib import Path
import re
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
BUCKETS = 128

def sha(data):
    return hashlib.sha256(data).hexdigest()

def base_key(key, language):
    if language == 'lat':
        key = re.sub(r'[_^]', '', key)
    match = re.fullmatch(r'(.*?)#?(\d+)', key)
    return (match[1], int(match[2])) if match else (key, None)

def bucket(key):
    value = 2166136261
    for byte in key.encode('utf-8'):
        value = ((value ^ byte) * 16777619) & 0xffffffff
    return f'{value % BUCKETS:02x}'

def text(value):
    return html.escape(re.sub(r'\s+', ' ', value or ''), quote=False)

def render(element, dictionary, inherited=None):
    """Retain all textual children, using only a closed set of passive HTML tags."""
    language = element.get('lang', inherited)
    tag = element.tag
    if tag in ('pb', 'cb'):
        return ''
    content = text(element.text) + ''.join(render(child, dictionary, language) + text(child.tail) for child in element)
    out = 'span'
    classes = []
    attrs = ''
    if tag == 'sense':
        out = 'div'; classes.append('dictionary-sense')
        if element.get('n'):
            content = '<strong class="sense-label">' + text(element.get('n')) + '</strong> ' + content
    elif tag in ('p', 'head'):
        out = 'div'
    elif tag == 'orth':
        out = 'strong'
    elif tag in ('tr', 'gloss') or tag == 'hi' and element.get('rend') == 'ital':
        out = 'em'
    if element.get('lang'):
        lang = {'greek': 'grc', 'la': 'la', 'en': 'en', 'he': 'he', 'fr': 'fr'}.get(language)
        if lang:
            attrs += ' lang="' + lang + '"'
        if language == 'greek' and dictionary == 'lsj':
            classes.append('beta-text')
    if classes:
        attrs += ' class="' + ' '.join(classes) + '"'
    return '<' + out + attrs + '>' + content + '</' + out + '>'

def preview(element, dictionary):
    """Keep a contiguous original-text context, never a list of isolated glosses."""
    segments = []
    position = 0
    focus = None
    def walk(e, language=None, excluded=False):
        nonlocal position, focus
        language = e.get('lang', language)
        excluded = excluded or e.tag in ('cit', 'bibl', 'etym', 'quote', 'xr')
        selected = e.tag in ('tr', 'gloss') if dictionary == 'lsj' else e.tag == 'hi' and e.get('rend') == 'ital'
        value = re.sub(r'\s+', ' ', ''.join(e.itertext())).strip()
        if selected and not excluded and focus is None and value and not re.fullmatch(r'(?:[A-Za-z]{1,6}\.\s*)+', value):
            focus = position
        def add(value, language):
            nonlocal position
            value = re.sub(r'\s+', ' ', value or '')
            if value:
                segments.append({'text': value, 'language': language, 'start': position})
                position += len(value)
        add(e.text, language)
        for child in e:
            walk(child, language, excluded)
            add(child.tail, language)
    walk(element)
    full = ''.join(s['text'] for s in segments)
    if focus is None:
        focus = 0
    start = max(0, focus - 160)
    if start:
        space = full.rfind(' ', 0, start)
        start = space + 1 if space >= 0 else 0
    end = min(len(full), focus + 560)
    if end < len(full):
        space = full.find(' ', end)
        end = space if space >= 0 else len(full)
    result = []
    if start:
        result.append({'text': '… ', 'language': None})
    for segment in segments:
        a, b = max(start, segment['start']), min(end, segment['start'] + len(segment['text']))
        if a < b:
            result.append({'text': segment['text'][a-segment['start']:b-segment['start']], 'language': segment['language']})
    if end < len(full):
        result.append({'text': ' …', 'language': None})
    return result

def build():
    lock = json.loads((ROOT / 'dictionary-sources.lock.json').read_text())
    destination = ROOT / 'dictionaries'
    destination.mkdir(exist_ok=True)
    manifest = {'schema': 'morph-dictionaries/1', 'adapter': 'morph-lexicon/1', 'source': lock['revision'],
                'license': 'CC-BY-SA-4.0', 'buckets': BUCKETS, 'dictionaries': {}}
    report = {}
    for dictionary, source in lock['dictionaries'].items():
        shards = [{} for _ in range(BUCKETS)]
        ids = set(); count = 0; references = 0; duplicates = 0
        for item in source['files']:
            packed = (ROOT / item['path']).read_bytes()
            assert sha(packed) == item['gzipSha256'], item['path']
            original = gzip.decompress(packed)
            assert sha(original) == item['sha256'], item['path']
            # ElementTree does not fetch the external DTD; all sources must parse as retained.
            for event, entry in ET.iterparse(io.BytesIO(original), events=('end',)):
                if entry.tag != 'entryFree':
                    continue
                key, entry_id = entry.attrib['key'], entry.attrib['id']
                identity = item['sourcePath'] + '#' + entry_id
                assert identity not in ids, identity
                ids.add(identity)
                base, number = base_key(key, source['language'])
                record = {'id': entry_id, 'key': key, 'number': 1 if number is None else number,
                          'source': item['sourcePath'], 'html': render(entry, dictionary),
                          'preview': preview(entry, dictionary)}
                index = int(bucket(base), 16)
                shards[index].setdefault(base, []).append(record)
                count += 1
                entry.clear()
        directory = destination / dictionary
        directory.mkdir(exist_ok=True)
        files = []
        for index, records in enumerate(shards):
            data = json.dumps(records, ensure_ascii=False, separators=(',', ':')).encode()
            packed = gzip.compress(data, compresslevel=9, mtime=0)
            name = f'{dictionary}/{index:02x}.json.gz'
            (destination / name).write_bytes(packed)
            files.append({'path': name, 'bytes': len(packed), 'sha256': sha(packed)})
            duplicates += sum(len(v) > 1 for v in records.values())
        manifest['dictionaries'][dictionary] = {'title': source['title'], 'language': source['language'], 'entries': count, 'files': files}
        report[dictionary] = {'entries': count, 'keysWithMultipleEntries': duplicates, 'compressedBytes': sum(f['bytes'] for f in files)}
    (destination / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps(report, indent=2))

if __name__ == '__main__':
    build()
