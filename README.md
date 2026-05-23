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

## Prerequisites

Before you can use `besmc` you need two things installed:

1. **The dubna simulator** — download and install it from
   [github.com/besm6/dubna](https://github.com/besm6/dubna/).
   The `dubna` command must be on your `$PATH`.

2. **The Rust compiler** — needed only to build `besmc` itself.
   Install it from [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Build and Install

```sh
make
make install
```

The `besmc` binary is installed to `~/.cargo/bin/besmc`.

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

## Command-Line Options

| Option | Description |
| --- | --- |
| `-c` / `--compile` | Compile to an object file (`.obj`); do not link |
| `-o FILE` / `--output FILE` | Set the output file name (default: derived from the first input file) |
| `-t` / `--save-temps` | Keep intermediate files (`.dub` script, `output.bin`, `persNN.bin`) |
| `-h` / `--help` | Print help |

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
