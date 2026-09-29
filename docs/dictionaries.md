# Original dictionary editions and the presentation adapter

The 0.0.2 application mounts complete **LSJ (116,497 entries)** and **Lewis &
Short (51,645 entries)** from PerseusDL/lexica revision
`56061ca127f4a2844980baffc5f2b6d1332897b3`, retrieved 2026-09-29.
These are uncollated Perseus digital editions, not new critical editions or a
claim about the current contents of the Tufts production database.

## Source and license

`dictionary-sources.lock.json` identifies all 27 LSJ XML files and the Unicode
Greek version of Lewis & Short (`lat.ls.perseus-eng2.xml`). Its archival Beta Code
counterpart is not counted as a second dictionary. The unchanged original bytes
are stored as reproducible gzip in `vendor/lexica/`, including full headers and
editorial history. Decompressing each file reproduces the recorded source hash.

The XML and derived display packs retain **CC BY-SA 4.0**, their original credit
and availability statements in `licenses/perseus-lsj.md` and `perseus-ls.md`.
They are separate from the application's CC BY-SA 3.0 US code. The complete
conversion and unchanged source are offered publicly to Perseus and all recipients.

## Join contract: morph-lexicon/1

The C engine and `morph-engine/1` record do not change. A presentation parser reads
`<NL>` records. `DumpPerseusAnalysis` (`src/anal/prntanal.c`) can print a changed
working form followed by a comma before the lemma. The historical Hopper
`Cruncher.java` regex extracts the final comma-delimited field. This adapter
uses that display convention, retains the entire original analysis in the raw
record, and shows the preceding working-form field separately. Unresolved complex
prefix labels are not repaired or guessed.

Greek lemmas query LSJ; Latin lemmas query Lewis & Short. The adapter removes
Latin quantity markers `_` and `^` from lookup keys, preserves case/Greek accents,
and separates optional `#` plus trailing homonym digits. Entry keys without a
number have sequence 1, following the historical dictionary loader. A numbered
engine lookup matches only that sequence; an unnumbered lookup returns all source
entries under that exact headword, in retained source order. For example,
`gallus#1` resolves to L&S `n19272`; unnumbered `Gallus` returns entries 2, 3 and 4,
not an invented first homonym. The stored identity is dictionary + source revision
+ XML file + original entry ID, never the morphology analysis counter.

There is no accent/case folding, prefix removal, fuzzy match, silent alternative
lemma, contextual sense selection or redirection of an absent entry. A missing
match and a failed dictionary download have different messages; the latter can
be retried without rerunning or modifying the morphology. Thus full source-pack
coverage is not a claim that every engine lemma has a matching dictionary entry.

## Text and presentation

The builder retains all entry text and its order, sense numbering, emphasis and
citations in a passive display vocabulary. Page/column milestone elements and
non-display TEI attributes are left in the original XML. LSJ's explicitly tagged
Greek Beta Code is converted by the existing display converter; the Latin source
already contains Unicode Greek. Source cross-references/citations remain text,
not remote runtime requests. The source XML and its IDs remain available for
checking unusual glyphs, markup and transcription errors.

The reading preview is a contiguous original-text excerpt around the first
marked translation (LSJ `tr`/`gloss`) or non-abbreviation italic passage (L&S
`hi`), outside quotations, citations, bibliography and etymology. It retains
adjacent qualifications, oppositions, examples and language tags. Ellipses mark
omitted surrounding text; the complete entry is available next to it. It does
not turn isolated italic words into a list of meanings: for example, the
opposition to gods in the entry for ἄνθρωπος must remain an opposition, not an
additional definition. No summarization, editorial correction or translation
is applied.

The application inserts text and a closed set of passive DOM elements, not arbitrary
source HTML. Source URLs never execute during dictionary display.

## Build and offline behavior

`python3 scripts/build-dictionaries.py` verifies original hashes, parses every
entry and builds 128 deterministic gzip JSON shards per dictionary, indexed by
FNV-1a of the exact headword's UTF-8 bytes. `dictionaries/manifest.json` records
counts, lengths and SHA-256 hashes. A word selection verifies and decompresses its
shard locally. At most six decoded shards remain in the client cache; no whole
XML document is loaded into the browser. Failed loads are not cached as misses.

**Save for offline** includes all 256 shards, the manifest, core and fonts in one
verified complete edition. A browser cold restart with the origin unavailable
must still display definitions for words not previously selected. The static
archive contains the same complete runtime; the source archive additionally
contains all XML and the conversion script. Large archives are downloaded from
GitHub/Zenodo, while runtime shards remain same-origin static assets.
