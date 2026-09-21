# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`besmc` is a compiler frontend for the BESM-6 Soviet mainframe. It accepts source files in various languages (Algol, Fortran, Pascal, B, and assemblers) and produces BESM-6 executables or object files. The backend is the [`dubna`](https://github.com/besm6/dubna/) simulator, which runs the native BESM-6 compilers internally.

External tools required at runtime:
- `dubna` — the BESM-6 simulator (must be installed and on `$PATH`)
- `pascompl` — Pascal-re compiler (required only for `*.pas` files)
- `cpp`, `b6parse`, `b6lower`, `b6codegen` — the C preprocessor and the three BESM-6 C
  compiler passes (required only for `*.c` files). The BESM-6 C headers and `libc.bin`
  runtime are discovered at runtime under `~/.local/share/besm6/`, `/usr/local/share/besm6/`,
  or `/usr/share/besm6/` (`include/` and `lib/libc.bin`).

## Commands

```bash
make          # Build (g++ -std=c++20)
make test     # Run all tests
make install  # Install to ~/.local/bin/besmc
make clean    # Remove build artifacts

# Build and run directly:
./build/besmc hello.pascal
```

## Architecture

C++20 sources under `src/`:

### [src/main.cpp](src/main.cpp)
Entry point. Parses CLI via `parse_args()`, calls `compile_files()`, catches exceptions and exits 1.

### [src/besmc.hpp](src/besmc.hpp) / [src/besmc.cpp](src/besmc.cpp)
All compilation logic lives in `compile_files()`:

1. **`.pas` pre-processing** — For each `*.pas` input, runs `pascompl -P <file> <file.std>` and substitutes the `*.std` path in the file list.
1a. **`.c` pre-processing** — For each `*.c` input, runs the four-pass C pipeline and substitutes the resulting `*.madlen` path in the file list (which then flows through the normal `*madlen` path). Intermediate names are formed by *appending* to the full source name (`hello.c` → `hello.c.i`/`.asn`/`.tac`/`.madlen`) so they keep the `.madlen` extension yet never clobber an unrelated hand-written `hello.madlen`:
   - `cpp -E -nostdinc -I<include_dir> hello.c hello.c.i` — the include dir is discovered at runtime (see runtime tools above). The traditional positional `infile outfile` form with a joined `-I<dir>` is used so it works whether `cpp` is GNU cpp or clang.
   - `b6parse hello.c.i hello.c.asn` → `b6lower hello.c.asn hello.c.tac` → `b6codegen --madlen hello.c.tac hello.c.madlen`.
2. **Dubna script generation** — Writes a `*.dub` script that the `dubna` simulator will interpret:
   - `*file:persNN` directives map object files to virtual "perso" devices (octal addresses 40–57).
   - When any `.b` source is present, `*tape:7/b,40` and `*library:40` are inserted before `*call setftn` to load the B compiler tape and runtime library.
   - When any `.c` source is present and the output is an executable, libc is linked in: `*file:libc,37` is emitted in the mount section (its `libc.bin` is symlinked into the cwd for the duration of the run), and `*library:37` is emitted just before `*library:22` in the link step. Device 37 sits just below the perso range (40–57) so it never collides with `.obj` mounts. Not added for `-c` object output, which does not link.
   - Each source file is embedded inline with its language directive (`*ftn`, `*pascal`, `*algol`, `*madlen`, `*bemsh`, `*trans-main:40020` for B, etc.) or included via `*call perso:NN,cont` for `.obj` files.
   - The final step is either `*call to perso:60` (for `-c` / object output) or `*library:22` + `*call overlay` + entry point (for executable output).
3. **Running Dubna** — Invokes `dubna <script.dub>` with stdout redirected to the listing file (`*.lst`).
4. **Error detection** — Scans the listing with regex patterns for Russian-language BESM-6 error messages. Any match causes a compilation failure.
5. **Output** — Copies `output.bin` to the final file. For executables, prepends a `#!/usr/bin/env dubna` shebang and sets the executable bit.

**Entry point selection**: БЕМШ (`.bemsh`) programs use `main` as the overlay entry; all other languages use `program`.

**Object file limit**: At most 16 object files can be linked (perso devices 040–057 octal).

### [tests/tests.cpp](tests/tests.cpp)
Single sequential test harness covering CLI parsing, per-language `.exe`/`.obj` builds, negative cases, Pascal→Fortran linking, and `*.std` input. C tests are skipped when `b6parse` is not on `$PATH`.

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
| `.c` | C language → `cpp` + `b6parse`/`b6lower`/`b6codegen`, produce `.madlen`; links `libc.bin` via device 37 (modern port, not original BESM-6) |
| `.obj` | Object library (`*call perso:NN,cont`) |
| `.std` | Standard array output of `pascompl` (passed through verbatim) |

## Intermediate Files

When a compilation runs, these temporary files appear in the working directory and are deleted on success (unless `-t`/`--save-temps` is set):

- `<output>.dub` — the generated Dubna script
- `output.bin` — raw binary produced by Dubna
- `persNN.bin` — copies of `.obj` inputs mounted as virtual perso devices
- `<src>.c.i`, `<src>.c.asn`, `<src>.c.tac`, `<src>.c.madlen` — intermediates from the `.c` pipeline (named next to each `.c` source by appending to the full source name)
- `libc.bin` — symlink to the BESM-6 libc, created in the cwd while linking C code
