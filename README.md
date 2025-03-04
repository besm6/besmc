The `besmc` command is a compiler fontend for BESM-6 machine.
It translates Algol, Fortran or Pascal source code into executable programs.
It also supports assemblers Madlen and БЕМШ.
On the backend, the `dubna` simulator runs native BESM-6 compilers.

## Basic Usage ##

The general syntax is:
```
$ besmc
BESM-6 compiler frontend

Usage: besmc [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Input sources and object files:
              *.ftn     - Fortran-ГДP
              *.fortran - Fortran Dubna
              *.forex   - Forex
              *.algol   - Algol-ГДP
              *.pascal  - Pascal
              *.assem   - Assembler Madlen
              *.madlen  - Assembler Madlen-3.5
              *.bemsh   - Assembler БЕМШ
              *.obj     - Object Library (*perso)

Options:
  -o, --output <FILE>  Output file name
  -c, --compile        Compile only to object files
  -t, --save-temps     Keep intermediate files
  -h, --help           Print help
```
To use `besmc` you need [dubna](https://github.com/besm6/dubna/) installed.

## Example ##
For a simple "Hello, World!" program in a file called `hello.pascal`:
```
program main(output);
_(
    writeln('Hello, Pascal!');
    stop;
_).
```
Run:

    besmc hello.pascal

Then execute the program:

    $ ./hello.exe
    HELLO, PASCAL!

When compiling the program, a listing file is also generated, with `.lst` extension.
For the above example, it would be [hello.lst](https://gist.github.com/sergev/564571bd708d2d016892143270aad968).

More examples are available:

 * [examples/hello.algol](examples/hello.algol)
 * [examples/hello.assem](examples/hello.assem)
 * [examples/hello.bemsh](examples/hello.bemsh)
 * [examples/hello.forex](examples/hello.forex)
 * [examples/hello.fortran](examples/hello.fortran)
 * [examples/hello.ftn](examples/hello.ftn)
 * [examples/hello.madlen](examples/hello.madlen)
 * [examples/hello.pascal](examples/hello.pascal)

## Mixed-language programming ##

A compilation of different languages into one program is possible.
For example, consider a Pascal program which calls a Fortran routine.

Caller in Pascal:
```
program main (output);
procedure hello; fortran;
_(
    hello;
    stop;
_).
```
Routine in Fortran:
```
        subroutine hello
        print 1000
 1000   format('Hello Fortran from Pascal!')
        end
```
Compile and run:

    $ besmc -c caller.pascal
    $ besmc -c callee.ftn
    $ besmc caller.obj callee.obj
    $ ./caller.exe
    HELLO FORTRAN FROM PASCAL!

## Build and install ##

To build besmc you must have the Rust compiler available.
For information on how to install the Rust compiler, see:
[rust-lang.org](https://www.rust-lang.org/tools/install).

Run:

    make
    make install

The binary will be installed as `$HOME/.cargo/bin/besmc`.

## Tests ##

To validate the compiler, run:

    make test

You should see:
```
$ make test
cargo test -- --test-threads=1
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/main.rs (target/debug/deps/besmc-11b48d6255548305)

running 51 tests
test test::test_exe::test_algol_exe ... ok
test test::test_exe::test_assem_exe ... ok
...
test test::test_pascal_to_fortran::test_pascal_to_fortran ... ok

test result: ok. 51 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.16s
```
