# Morph 0.0.2 — passage reading and offline dictionaries

Entering a passage now shows the original text, with punctuation, line breaks and
repeated words preserved. Select a word to see its meanings and original forms;
a List view remains available. Keyboard selection and view switching are supported.

This edition mounts complete, uncollated Perseus dictionaries: LSJ (116,497
entries) and Lewis & Short (51,645). The source XML, IDs, source revision, original
notices and reproducible conversion are included. Entries provide short labelled
excerpts and the complete text with citations. Dictionary joins respect numbered
homonyms, preserve explicit missing matches, and distinguish a failed download
from an absent entry. The original output's optional working-form prefix is now
separated from the dictionary headword in presentation.

The C/Wasm artifacts, morphology data, engine protocol and raw execution output
are unchanged from 0.0.1. This frontend and dictionary integration is not scholarly
collation and does not begin the Rust translation or assert 1.0 qualification.

Save for offline includes both complete dictionaries, bringing the application
and offline assets to about 62 MB. Queries need no server after that save. Large
source and static archives are held independently on GitHub and Zenodo.

Validation includes exact-passage and occurrence selection, Greek/Latin meanings,
full-entry rendering, homonyms, raw-output retention, corruption/retry, three
browser engines, and browser-process restart with the origin unavailable. The
retained 2,000-form corpus also measures dictionary joins without guessing missing
matches; see `evidence/dictionary-corpus.json`. Physical mobile-device acceptance
remains separate from the 320–1280px browser checks.

Code/core license: CC BY-SA 3.0 US. Dictionary source and derived display packs:
CC BY-SA 4.0. Original Perseus attribution and availability notices are retained;
runtime/font components retain their own licenses. This independent preservation
project implies no Tufts/Perseus endorsement.
