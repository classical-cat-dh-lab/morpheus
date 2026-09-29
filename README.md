# Morph

Ancient Greek and Latin morphology running locally in a modern browser.
An independent preservation of Gregory Crane's **Morpheus**, from the Perseus
Digital Library at Tufts University. The engineering edition uses the original
C program compiled to WebAssembly; it does not require a live Perseus service.

Version **0.0.2** adds passage selection and complete offline LSJ/Lewis & Short dictionaries
to the unchanged 0.0.1 engine interface.
It is an engineering reconstruction, not a completed source-faithful Rust port or
a claim of exact equivalence to the presently deployed Perseus service.

The development branch also contains a [Rust preservation candidate](rust/README.md).
It is runnable, but a [known internal-fidelity blocker](rust/BLOCKERS.md) prevents
replacement of the released engine. It is not a completed 1.0 port.

## Use and keep

Open [morph.latingreek.org](https://morph.latingreek.org/), choose Greek or Latin, and enter a word or passage.
Greek Unicode is explicitly converted to Beta Code. Latin macrons/breves are
removed only from the delivered input; the original text and conversion record
remain available. Original input mode passes ASCII bytes through without rewriting.

Choose **Save for offline** and wait for **Ready offline**. This saves the entire
application, fonts, morphology data and both dictionaries (about 62 MB) after verifying their hashes. A previous
complete version survives interrupted updates. Browsers can clear stored data;
download the source and static release archives as independently held copies.
PWA installation and a verified complete offline copy are distinct states.

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

## Scope of this edition

- Original `cruncher` analysis for Greek and Latin; raw stdout/stderr and execution
  records; readable morphology with original lemma labels and candidate order.
- A versioned byte interface, fresh process state per job, cancellation, a CLI,
  and bounded original file-mode access through the API/CLI.
- Clickable passage and list views, complete Perseus LSJ and Lewis & Short
  entries with original source IDs; no scholarly corrections or guessed mappings.
  See [dictionary contract](docs/dictionaries.md).
- Up to 1,000 lines, 64 KiB per job and 48 ASCII bytes per line; unsupported input
  is rejected before entering legacy fixed-size C buffers. No silent truncation.
- Source pin `PerseusDL/morpheus@b1b33c56ef2338fe0dcd1893628ed638f00c0986`.
  Latin is reconstructed from its four retained verb inputs; the recipe's missing
  `vbs.mpi` is not fabricated or replaced with another fork's data.

Generation and historical companion executables remain in the source inventory,
but are not advertised browser operations in 0.0.2. Full preservation qualification
and the later Rust/Wasm implementation remain separate work toward 1.0.

## License and credit

CC BY-SA 3.0 US for the inherited core/data and this edition's adaptation and new
code, with separately licensed CC BY-SA 4.0 dictionaries and runtime/font components. See [LICENSE](LICENSE).
The complete modifications are publicly offered to Perseus and other recipients.
This is an independent Lab project; no Tufts/Perseus endorsement is implied.

Use [CITATION.cff](CITATION.cff) to cite this edition (DOI: [10.5281/zenodo.23043494](https://doi.org/10.5281/zenodo.23043494)). Preserve the original Morpheus
attribution when citing, adapting or redistributing the program and its data.
