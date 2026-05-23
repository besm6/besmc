# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`besmc` is a compiler frontend for the BESM-6 Soviet mainframe. It accepts source files in various languages (Algol, Fortran, Pascal, B, and assemblers) and produces BESM-6 executables or object files. The backend is the [`dubna`](https://github.com/besm6/dubna/) simulator, which runs the native BESM-6 compilers internally.

External tools required at runtime:
- `dubna` — the BESM-6 simulator (must be installed and on `$PATH`)
- `pascompl` — Pascal-re compiler (required only for `*.pas` files)

## Commands

```bash
make          # Build (cargo build)
make test     # Run all tests (must be single-threaded)
make install  # Install to ~/.cargo/bin/besmc
make clean    # Remove build artifacts

# Run a single test by name:
cargo test -- --test-threads=1 test_pascal_exe

# Build and run directly:
cargo run -- hello.pascal
```

Tests **must** run with `--test-threads=1` because they share the working directory and write temporary files there.

## Architecture

The codebase has two source files and a test tree:

### [src/main.rs](src/main.rs)
Entry point. Defines `CompilerOptions` (parsed by `clap` with `#[derive(Parser)]`), sets a silent panic hook, and calls `compile_files()` inside `panic::catch_unwind`. Fatal errors are signalled by `panic!()` throughout the codebase and caught here to print a clean message and `exit(1)`.

### [src/compiler.rs](src/compiler.rs)
All compilation logic lives in `compile_files()`:

1. **`.pas` pre-processing** — For each `*.pas` input, runs `pascompl -P <file> <file.std>` and substitutes the `*.std` path in the file list.
2. **Dubna script generation** — Writes a `*.dub` script that the `dubna` simulator will interpret:
   - `*file:persNN` directives map object files to virtual "perso" devices (octal addresses 40–57).
   - When any `.b` source is present, `*tape:7/b,40` and `*library:40` are inserted before `*call setftn` to load the B compiler tape and runtime library.
   - Each source file is embedded inline with its language directive (`*ftn`, `*pascal`, `*algol`, `*madlen`, `*bemsh`, `*trans-main:40020` for B, etc.) or included via `*call perso:NN,cont` for `.obj` files.
   - The final step is either `*call to perso:60` (for `-c` / object output) or `*library:22` + `*call overlay` + entry point (for executable output).
3. **Running Dubna** — Invokes `dubna <script.dub>` with stdout redirected to the listing file (`*.lst`).
4. **Error detection** — `search_errors_in_listing()` scans the listing with a set of compiled regex patterns for Russian-language BESM-6 error messages. Any match causes a compilation failure.
5. **Output** — Copies `output.bin` to the final file. For executables, prepends a `#!/usr/bin/env dubna` shebang and sets the executable bit.

**Entry point selection**: БЕМШ (`.bemsh`) programs use `main` as the overlay entry; all other languages use `program`.

**Object file limit**: At most 16 object files can be linked (perso devices 040–057 octal).

### [src/test/](src/test/)
Tests are split into modules under `src/test/`:

| Module | What it tests |
|---|---|
| `test_exe` | Compile each language → `.exe`, verify listing summary line |
| `test_obj` | Compile each language with `-c` → `.obj` |
| `test_negative` | Bad source → compilation must panic |
| `test_obj_negative` | Bad source with `-c` → must panic |
| `test_options` | CLI argument parsing |
| `test_pascal_to_fortran` | Two-step compile then link across languages |
| `test_stdarray` | Compile a pre-processed `*.std` file |

Test helpers in `src/test/mod.rs`:
- `parse_and_process(args)` — wraps `CompilerOptions::try_parse_from`
- `find_line_starting_with(filename, prefix)` — scans a listing file and returns the first matching line (used to assert on BESM-6 output size/summary lines)

Working examples for every language are in [examples/](examples/) — see [examples/README.md](examples/README.md) for annotated source, compiler listing excerpts, and expected output for each one.

## File Extensions and Language Mapping

| Extension | Language / Tool |
|---|---|
| `.ftn` | Fortran-ГДP (`*ftn`) |
| `.fortran` | Fortran Dubna (`*fortran`) |
| `.forex` | Forex (`*forex`) |
| `.algol` | Algol-ГДP (`*algol`) |
| `.pascal` | Pascal (`*pascal`) |
| `.pas` | Pascal-re → run `pascompl`, produce `.std` |
| `.assem` | Assembler Madlen (`*assem`) |
| `.madlen` | Assembler Madlen-3.5 (`*madlen`) |
| `.bemsh` | Assembler БЕМШ (`*bemsh`) |
| `.b` | B language (`*trans-main:40020`; requires `*tape:7/b,40` preamble; modern port, not original BESM-6) |
| `.obj` | Object library (`*call perso:NN,cont`) |
| `.std` | Standard array output of `pascompl` (passed through verbatim) |

## Intermediate Files

When a compilation runs, these temporary files appear in the working directory and are deleted on success (unless `-t`/`--save-temps` is set):

- `<output>.dub` — the generated Dubna script
- `output.bin` — raw binary produced by Dubna
- `persNN.bin` — copies of `.obj` inputs mounted as virtual perso devices
