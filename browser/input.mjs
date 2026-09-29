export const converter = 'morph-input/1';
const greek = 'αβγδεζηθικλμνξοπρστυφχψω';
const beta =  'abgdezhqiklmncoprstufxyw';
const letters = new Map([...greek].map((letter, i) => [letter, beta[i]]));
letters.set('ς', 's');
const marks = new Map([['\u0313', ')'], ['\u0314', '('], ['\u0301', '/'], ['\u0300', '\\'],
  ['\u0342', '='], ['\u0308', '+'], ['\u0345', '|'], ['\u0304', '_'], ['\u0306', '^']]);
const reverse = new Map([...marks].map(([a,b]) => [b,a]));

export function toBeta(word) {
  let result = '';
  if (/^\p{M}/u.test(word.normalize('NFD'))) throw new Error('A Greek combining mark must follow a letter.');
  const clusters = word.normalize('NFD').match(/\P{M}\p{M}*/gu) ?? [];
  for (const cluster of clusters) {
    const [letter, ...diacritics] = [...cluster];
    if (['\'', '’', '᾽', 'ʼ'].includes(letter) && !diacritics.length) { result += "'"; continue; }
    const base = letters.get(letter.toLowerCase());
    if (!base) throw new Error(`Unsupported Greek character: ${letter}`);
    const ordered = diacritics.map(mark => {
      if (!marks.has(mark)) throw new Error(`Unsupported diacritic in ${word}`);
      return marks.get(mark);
    });
    // Fixed semantic order: breathing, diaeresis, accent, quantity, subscript.
    ordered.sort((a,b) => ')(+/\\=_^|'.indexOf(a) - ')(+/\\=_^|'.indexOf(b));
    const uppercase = letter !== letter.toLowerCase();
    result += uppercase ? '*' + ordered.join('') + base : base + ordered.join('');
  }
  return result;
}

export function fromBeta(text) {
  return text.replace(/\*?[)(+/\\=_^|]*[a-z][)(+/\\=_^|]*/g, (part, index) => {
    const base = part.match(/[a-z]/)?.[0], position = beta.indexOf(base);
    if (position < 0) return part;
    let letter = greek[position];
    if (base === 's' && !/[a-z]/.test(text[index + part.length] ?? '')) letter = 'ς';
    if (part.includes('*')) letter = letter.toUpperCase();
    const accents = [...part].filter(c => reverse.has(c)).map(c => reverse.get(c)).join('');
    return (letter + accents).normalize('NFC');
  });
}

export function prepareInput(original, language, mode = 'unicode') {
  if (!['grc', 'lat'].includes(language)) throw new Error('Select Greek or Latin.');
  if (!['unicode', 'original'].includes(mode)) throw new Error('Unsupported input mode.');
  const records = [];
  let delivered;
  if (mode === 'original') {
    if (/[^\x09\x0A\x0D\x20-\x7E]/.test(original)) throw new Error('Original mode accepts ASCII Beta Code / Latin, one word per line.');
    delivered = original;
    for (const match of original.matchAll(/[^\r\n]+/g)) records.push({original: match[0], input: match[0], start: match.index, end: match.index + match[0].length});
  } else {
    for (const match of original.matchAll(/[\p{L}\p{M}]+(?:['’᾽ʼ][\p{L}\p{M}]*)?/gu)) {
      const word = match[0];
      let input;
      if (language === 'grc') input = toBeta(word);
      else {
        input = word.normalize('NFD').replace(/[\u0304\u0306]/g, '').replace(/[’᾽ʼ]/g, "'");
        if (/[^A-Za-z']/.test(input)) throw new Error(`Unsupported Latin character in ${word}. Use original mode for literal engine input.`);
      }
      records.push({original: word, input, start: match.index, end: match.index + word.length});
    }
    delivered = records.map(record => record.input).join('\n') + (records.length ? '\n' : '');
  }
  return {converter, language, mode, original, records, delivered, bytes: new TextEncoder().encode(delivered)};
}
