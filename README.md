# Morph

Ancient Greek and Latin morphology running locally in a modern browser.
An independent preservation of Gregory Crane's **Morpheus**, from the Perseus
Digital Library at Tufts University. **Version 0.1.0 is the Rust beta**: the
[accepted and frozen preservation core](rust/ACCEPTANCE.md) now runs through
WebAssembly in the application, without a live Perseus service.

The declared Greek/Latin process boundary preserves raw I/O and source-led internal
behavior against the retained portable C reference, including the original
[retry-memory defect](rust/CAUSALITY.md). This does not claim exact equivalence to
the present Tufts server. The original engine, data, profile and byte protocol
remain traceable; there is no C fallback. Frontend refinement continues toward 1.0.

## Use and keep

Open [morph.latingreek.org](https://morph.latingreek.org/), choose Greek or Latin, and enter a word or passage.
Greek Unicode is explicitly converted to Beta Code. Latin macrons/breves are
removed only from the delivered input; the original text and conversion record
remain available. Original input mode passes ASCII bytes through without rewriting.

Choose **Save for offline** and wait for **Ready offline**. This saves the entire
application, fonts, morphology data and both dictionaries (about 63 MB) after verifying their hashes. A previous
complete version survives interrupted updates. Browsers can clear stored data;
download the source and static release archives as independently held copies.
Install Morph using your browser's Install app / Add to Home Screen command.
Installation and a verified complete offline copy are distinct states; both are
recommended for regular use. Opening the URL remains immediately usable, with
all analysis on your own device.

After saving, **Update & offline** opens compact maintenance controls. Check for
updates fetches only a manifest. A download reuses hash-verified unchanged data,
checks the saved Greek and Latin analyzer, and offers **Reload with update** when
ready. Existing reading sessions retain their selected engine. No lookup account,
server query allowance or periodic online authorization is required.

The static archive works on ordinary HTTPS static hosting. End users need neither
a compiler nor a local server. Opening a file URL is not the PWA installation path.

## Run, verify and rebuild

Node 22 or newer runs the tests, CLI and local preview with no npm dependencies:

```sh
npm test
npm run site
npm run serve
printf 'lo/gos\n' | node scripts/cli.mjs -T
printf 'amo\n' | node scripts/cli.mjs -T -L
```

Rebuild the C tools, morphology data and Wasm with Python 3.14, Apple Clang 21,
make, flex, Perl and Emscripten 6.0.6. Supply an explicit SDK and a fresh directory:

```sh
python3 scripts/build-engine.py --output build/reference --emsdk /path/to/emsdk
```

The retained archive and `source.lock.json` fix the original input. The builder
verifies it, preserves the extracted source, writes adaptations separately in
build copies, and records tool versions, commands, data and output hashes.
`docs/compatibility.md` explains the runtime reference and bounded adaptations.
The browser engine artifacts are shipped so users do not need the toolchain.

Rebuild the dictionaries with Python 3.14 standard library:

```sh
python3 scripts/build-dictionaries.py
```

The unchanged XML files are retained as reproducible gzip files in `vendor/lexica/`.
`dictionary-sources.lock.json` fixes their provenance and hashes. Source and static
archives are built under `build/releases/<version>/`; published downloads are held
on GitHub and Zenodo. They exceed the static host’s individual asset limit.

The separate `morph-0.1.0-macos-arm64.tar.gz` companion contains the unchanged
qualified native Rust CLI and morphology data for Apple Silicon macOS. It needs
neither a compiler nor Node.js. See its included README for `MORPHLIB` and original
ASCII input. Other native platforms and CLI refinements are deferred.

The Rust build route and pinned dependencies are documented in [rust/README.md](rust/README.md).
The original C builder remains the preservation reference, not the shipped browser
implementation. `node scripts/check-rust-baseline.mjs` guards the accepted source
and evidence; the release's engine asset hashes identify the deployed core.

## Scope of this edition

- Original `cruncher` analysis for Greek and Latin; raw stdout/stderr and execution
  records; readable morphology with original lemma labels and candidate order.
- A versioned byte interface, fresh process state per job, cancellation, a CLI,
  and bounded original file-mode access through the API/CLI.
- Clickable passage and list views, complete Perseus LSJ and Lewis & Short
  entries with original source IDs; no scholarly corrections or guessed mappings.
  See [dictionary contract](docs/dictionaries.md).
- Up to 1,000 lines, 64 KiB per job and 48 ASCII bytes per line; unsupported input
  is rejected before entering legacy preserved fixed-size buffers. No silent truncation.
- Source pin `PerseusDL/morpheus@b1b33c56ef2338fe0dcd1893628ed638f00c0986`.
  Latin is reconstructed from its four retained verb inputs; the recipe's missing
  `vbs.mpi` is not fabricated or replaced with another fork's data.

Generation and historical companion executables remain in the source inventory,
but are not advertised browser operations in 0.1.0 or implied by the Rust core
acceptance. Frontend qualification continues toward 1.0.

## License and credit

CC BY-SA 3.0 US for the inherited core/data and this edition's adaptation and new
code, with separately licensed CC BY-SA 4.0 dictionaries and runtime/font components. See [LICENSE](LICENSE).
The complete modifications are publicly offered to Perseus and other recipients.
This is an independent Lab project; no Tufts/Perseus endorsement is implied.

Use [CITATION.cff](CITATION.cff) for this edition’s metadata (software concept DOI: [10.5281/zenodo.23042200](https://doi.org/10.5281/zenodo.23042200)). Preserve the original Morpheus
attribution when citing, adapting or redistributing the program and its data.
