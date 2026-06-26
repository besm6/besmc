# besmc — BESM-6 Compiler Frontend

## What is this?

**BESM-6** was a Soviet mainframe computer produced from the late 1960s through the 1980s.
It was one of the most powerful computers of its era in the Eastern Bloc, and it supported
a rich set of programming languages: Algol, Fortran, Pascal, and several assemblers. A modern
port of the B language (the predecessor of C) is also supported.

**`besmc`** is a command-line tool that lets you write programs in those same languages on
a modern computer and compile them into BESM-6 executables. Under the hood it uses the
[dubna](https://github.com/besm6/dubna/) simulator, which runs the original BESM-6 compilers
exactly as they ran on the real hardware.

In short: you write a program in one of the old BESM-6 languages, run `besmc yourprogram.pascal`,
and get a runnable `yourprogram.exe` — no real BESM-6 mainframe required.

## Contents

- [Prerequisites](#prerequisites)
- [Build and Install](#build-and-install)
- [Quick Start](#quick-start)
- [Supported Languages](#supported-languages)
- [Compiling to an Object File](#compiling-to-an-object-file)
- [Mixed-Language Programs](#mixed-language-programs)
- [Command-Line Options](#command-line-options)
- [Troubleshooting](#troubleshooting)
- [Running the Tests](#running-the-tests)

## Prerequisites

Before you can use `besmc` you need these installed:

1. **The dubna simulator** — download and install it from
   [github.com/besm6/dubna](https://github.com/besm6/dubna/).
   The `dubna` command must be on your `$PATH`. This is the engine that actually runs the
   BESM-6 compilers and your compiled programs, so it is required at all times.

2. **The Rust compiler** — needed only to build `besmc` itself.
   Install it from [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

3. **`pascompl`** — *only* needed if you compile `.pas` (Pascal-re) files. If you never use
   the `.pas` extension you can skip this one. It must also be on your `$PATH`.

## Build and Install

Get the source, build it, and install the command:

```sh
git clone https://github.com/besm6/besmc.git
cd besmc
make
make install
```

The `besmc` binary is installed to `~/.cargo/bin/besmc`.

**If you later type `besmc` and see `command not found`**, that directory is not on your
`$PATH`. Add it once:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

To make it permanent, add that same line to the end of your shell's startup file —
`~/.bashrc` for bash or `~/.zshrc` for zsh — then open a new terminal.

## Quick Start

Save this Pascal program as `hello.pascal`:

```pascal
program main(output);
_(
    writeln('Hello, Pascal!');
    stop;
_).
```

> **Note:** This Pascal dialect uses `_(` and `_).` as block delimiters instead of the
> usual `begin` / `end`.

Compile it:

```sh
besmc hello.pascal
```

Two files are created:

| File | What it is |
| --- | --- |
| `hello.exe` | The executable program |
| `hello.lst` | The compiler listing (output from the BESM-6 compiler, useful for diagnosing errors) |

Run the program:

```sh
$ ./hello.exe
HELLO, PASCAL!
```

> **What is that `.exe`?** Despite the name, it is **not** a Windows program. It is a tiny
> text script that begins with the line `#!/usr/bin/env dubna`, so running it simply hands it
> to the `dubna` simulator. It runs on any system (Linux, macOS, …) where `dubna` is installed,
> and you can also run it explicitly with `dubna hello.exe`.

## Supported Languages

`besmc` recognises the following file extensions:

| Extension | Language |
| --- | --- |
| `.pascal` | Pascal |
| `.pas` | Pascal-re (requires `pascompl` on your `$PATH`) |
| `.ftn` | Fortran-ГДP |
| `.fortran` | Fortran Dubna |
| `.forex` | Forex |
| `.algol` | Algol-ГДP |
| `.assem` | Assembler Madlen |
| `.madlen` | Assembler Madlen-3.5 |
| `.bemsh` | Assembler БЕМШ (source code written in Cyrillic) |
| `.b` | B (modern port) |
| `.obj` | Pre-compiled object library (for linking) |

Working "Hello, World!" examples for every language are in the [examples/](examples/) directory,
with detailed explanations in [examples/README.md](examples/README.md).

## Compiling to an Object File

By default `besmc` produces a ready-to-run `.exe`. If you want to compile a source file
without linking it (for example, to combine it later with code written in another language),
use the `-c` flag:

```sh
besmc -c hello.pascal
```

This produces `hello.obj` instead of `hello.exe`. Object files can be passed to a later
`besmc` invocation for linking.

## Mixed-Language Programs

One of BESM-6's strengths was that programs could freely mix languages — a Pascal main
program could call a Fortran subroutine, for instance. Here is how to do that with `besmc`.

**Step 1 — Write the Pascal main program** (`caller.pascal`):

```pascal
program main (output);
procedure hello; fortran;
_(
    hello;
    stop;
_).
```

The `procedure hello; fortran;` declaration tells the Pascal compiler that `hello` is
implemented in Fortran.

**Step 2 — Write the Fortran subroutine** (`callee.ftn`):

```fortran
        subroutine hello
        print 1000
 1000   format('Hello Fortran from Pascal!')
        end
```

**Step 3 — Compile each file separately, then link them together**:

```sh
besmc -c caller.pascal          # produces caller.obj
besmc -c callee.ftn             # produces callee.obj
besmc caller.obj callee.obj     # links both into caller.exe
```

Run the result:

```sh
$ ./caller.exe
HELLO FORTRAN FROM PASCAL!
```

> **Note:** You can list up to 16 object files in a single linking command.

## Command-Line Options

| Option | Description |
| --- | --- |
| `-c` / `--compile` | Compile to an object file (`.obj`); do not link |
| `-o FILE` / `--output FILE` | Set the output file name (default: derived from the first input file) |
| `-t` / `--save-temps` | Keep intermediate files (`.dub` script, `output.bin`, `persNN.bin`) |
| `-h` / `--help` | Print help |

## Troubleshooting

| Message you see | What it means and how to fix it |
| --- | --- |
| `dubna: command not found` | The dubna simulator is not installed or not on your `$PATH`. Install it from [github.com/besm6/dubna](https://github.com/besm6/dubna/) and make sure the `dubna` command works in your terminal. |
| `besmc: command not found` | The `besmc` binary is not on your `$PATH`. Add `~/.cargo/bin` to it — see [Build and Install](#build-and-install). |
| `Failed to execute pascompl` | You are compiling a `.pas` file but `pascompl` is not installed. Install it and put it on your `$PATH`, or use the `.pascal` extension instead, which does not need it. |
| `Compilation failed! See details in <name>.lst` | Your source code has an error. Open the `<name>.lst` listing file to find it. The BESM-6 compilers report errors in Russian (for example, lines containing `OШИБ` mean "errors"); the annotated listings in [examples/README.md](examples/README.md) show what a clean listing looks like for each language. |

**Tip:** When something goes wrong and you want to look under the hood, add `-t`
(`--save-temps`). `besmc` will then keep the intermediate files — including the generated
`*.dub` script that it feeds to dubna — instead of deleting them.

## Running the Tests

To verify that everything is working:

```sh
make test
```

You should see all tests pass:

```text
running 52 tests
test test::test_exe::test_pascal_exe ... ok
test test::test_exe::test_fortran_exe ... ok
...
test result: ok. 51 passed; 0 failed; 0 ignored
```
