# BESM-6 Example Programs

This directory contains working "Hello, World!" programs for every language supported by `besmc`,
plus a mixed-language example (Pascal calling a Fortran subroutine).

All examples are compiled by the original BESM-6 compilers running inside the
[dubna](https://github.com/besm6/dubna/) simulator — the same compilers that ran on the real
hardware in the 1970s and 1980s. As a result, all output appears in **uppercase**: that was the
character set of BESM-6 line printers.

## Pascal — `hello.pascal`

Pascal was the most modern high-level language available on BESM-6. The compiler dates from 1979.

```pascal
program main(output);
_(
    writeln('Hello, Pascal!');
_).
```

**Syntax notes:**

- Block delimiters are `_(` and `_).` instead of the standard `begin` / `end`.
- `output` in the program header declares that the program writes to the printer.

**Compile and run:**

```sh
besmc hello.pascal   # produces hello.exe and hello.lst
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
 PASCAL COMPILER. 14.(24.12.79)
 00001    1 PROGRAM MAIN(OUTPUT);
 00007    2 _(
 00007    3     WRITELN('HELLO, PASCAL!');
 00024    4 _).
...
 ДЛИHA БИБЛИOTEKИ  002 17
```

**Program output:**

```text
HELLO, PASCAL!
```

---

## Pascal-re — `hello.pas`

Pascal-re is a retargeted Pascal compiler. The source syntax is identical to `hello.pascal`,
but the `.pas` extension causes `besmc` to run `pascompl` first: a separate tool that
translates the source into a compact binary "standard array" (`.std` file), which is then
handed to dubna for final compilation.

```pascal
program main(output);
_(
    writeln('Hello, Pascal!');
_).
```

**Compile and run:**

```sh
besmc hello.pas      # calls pascompl internally, then dubna
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
*-CM-*    MAIN
PROGRA
...
 ДЛИHA БИБЛИOTEKИ  002 17
```

The `*-CM-*` marker shows that the standard array (pre-compiled module) was loaded
rather than source text being echoed by the compiler.

**Program output:**

```text
HELLO, PASCAL!
```

---

## Pre-processed Pascal — `stdarray.std`

This file is the pre-compiled output of `pascompl` — the binary "standard array" format
that Pascal-re programs are translated into before the BESM-6 Pascal compiler processes them.
Compiling a `.pas` file produces this format automatically as an intermediate step;
`stdarray.std` is a copy that has been saved for direct use.

The file was created with:

```sh
pascompl -P hello.pascal stdarray.std
```

The first few lines show the octal-encoded binary format:

```text
`77761 PROGRAM  1
`5541515600000000
`2000000000000000
`6062574762415500
...
```

**Compile and run:**

```sh
besmc stdarray.std
./stdarray.exe
```

**Program output:**

```text
HELLO, PASCAL!
```

---

## Algol-ГДP — `hello.algol`

Algol 60 was one of the first languages implemented on BESM-6. The "ГДP" (GDR) designation
means this is the East-German variant of the compiler, dating from 1982.

```algol
'begin'
    print(''Hello, Algol!'');
'end'
'eop'
```

**Syntax notes:**

- Language **keywords are enclosed in single quotes**: `'begin'`, `'end'`, `'eop'`.
- **String literals use doubled single quotes**: `''Hello, Algol!''`.
- Every Algol program must end with `'eop'` (end of program).
- The leading space before `HELLO, ALGOL!` in the output is added by the Algol print routine.

**Compile and run:**

```sh
besmc hello.algol
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
A L G O L - Г Д P
    (7.01.82)

     1.     'BEGIN'
     2.         PRINT('HELLO, ALGOL!');
     3.     'END'
     4.     'EOP'

PROGRAM   ДЛИHA:   16 00020B BPEMЯ:  0.00 CEK. KAPT:   4
...
 ДЛИHA БИБЛИOTEKИ  001 17
```

**Program output:**

```text
 HELLO, ALGOL!
```

---

## Fortran-ГДP — `hello.ftn`

Fortran-ГДP (GDR Fortran) is the East-German Fortran IV compiler, dated 1981. It is
the more modern of the two Fortran compilers available in dubna.

```fortran
        program hello
        print 1000
        stop
 1000   format('Hello, Ftn!')
        end
```

**Syntax notes:**

- This is **fixed-form Fortran**: columns 1–6 are for statement labels, column 7 for
  continuation, and columns 7–72 for the statement. Leading spaces push code to the
  statement column.
- `print 1000` prints according to the `format` statement labelled `1000`.
- `stop` halts the program explicitly.

**Compile and run:**

```sh
besmc hello.ftn
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
 Ф O P T P A H - Г Д P
      (09.07.81)
  001                 PROGRAM HELLO
  002                 PRINT 1000
  003                 STOP
  004          1000   FORMAT('HELLO, FTN!')
  005                 END

    HELLO   >00000<   PROGRA  >00000<
 >> HELLO  <<    LENGTH: 00009 00011B   INPUT CARDS: 00005
...
 ДЛИHA БИБЛИOTEKИ  002 30
```

**Program output:**

```text
HELLO, FTN!
```

---

## Fortran Dubna — `hello.fortran`

Fortran Dubna is the original Dubna Computing Centre Fortran IV compiler from 1973 — older
and slightly larger than Fortran-ГДP. The source syntax is identical to `.ftn`.

```fortran
        program hello
        print 1000
        stop
 1000   format('Hello, Fortran!')
        end
```

**Compile and run:**

```sh
besmc hello.fortran
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
 Ф O P T P A H
   /16.07.73/
                  PROGRAM HELLO
                  PRINT 1000
       2          STOP
           1000   FORMAT('HELLO, FORTRAN!')
                  END
...
 ДЛИHA БИБЛИOTEKИ  002 30
```

**Program output:**

```text
HELLO, FORTRAN!
```

---

## Forex — `hello.forex`

Forex is a Dubna-specific Fortran dialect. Its surface syntax is nearly identical to Fortran
but it is a separate compiler with a different code generator, developed at the Joint Institute
for Nuclear Research (Dubna, USSR) in 1985.

```fortran
        program hello
        print 1000
        stop
 1000   format('Hello, Forex!')
        end
```

**Compile and run:**

```sh
besmc hello.forex
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
 F O R E X ИПM AH CCCP 4.13 OT 11.09.85
    1                      PROGRAM HELLO
    2                      PRINT 1000
    3                      STOP
    4               1000   FORMAT('HELLO, FOREX!')
    5                      END

 KOMAHД 00013    KOHCTAHT 00003    ПAMЯTЬ ДЛЯ ПEPEMEHHЫX 00000
 ДЛИHA ПOДПPOГPAMMЫ HELLO  00016
...
 ДЛИHA БИБЛИOTEKИ  002 30
```

**Program output:**

```text
HELLO, FOREX!
```

---

## Assembler Madlen — `hello.assem`

Madlen (1972) is the older of the two BESM-6 macro-assemblers. The `.assem` extension
selects the original Madlen dialect.

```text
 program: ,name,
          ,*64 , info
          ,*74 ,
 info:    ,    , text
          ,    , text
          ,000 ,
         8,    ,
 text:    ,gost, 24hHello, Assembler!'214''231'
          ,end ,
```

**Syntax notes:**

- Instructions use a **comma-delimited column format**: `label: modifier, opcode, operand`.
- `*64` is the BESM-6 print instruction; `*74` advances to the next print line.
- `gost` selects the GOST character encoding (the Soviet standard character set).
- `24h...` is a **Hollerith string** of exactly 24 characters (the `h` prefix gives the length).
- `'214'` and `'231'` are **octal control codes** (carriage return and line feed).

**Compile and run:**

```sh
besmc hello.assem
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
 ABTOKOД  MADLEN
   (1.10.72)
                    PROGRAM :  , NAME,
 0000                          , *64 ,INFO
 0001                          , *74 ,
 0002               INFO    :  ,     ,TEXT
 0004               TEXT    :  , GOST,24HHELLO, ASSEMBLER!'214'
                                              '231'
 ЧИCЛO ПEPФ. 0009      ЧИCЛO OШИБ. OПEPATOPOB  0000
...
 ДЛИHA БИБЛИOTEKИ  001 01
```

The line `ЧИCЛO OШИБ. OПEPATOPOB  0000` ("number of erroneous operators: 0") confirms
the assembly was clean.

**Program output:**

```text
HELLO, ASSEMBLER!
```

---

## Assembler Madlen-3.5 — `hello.madlen`

Madlen-3.5 (1986) is the updated version of the Madlen assembler. The `.madlen` extension
selects this newer dialect. The instruction syntax is the same as `.assem`; the main
differences are improved diagnostics and a detailed structure report in the listing.

```text
 program: ,name,
          ,*64 , info
          ,*74 ,
 info:    ,    , text
          ,    , text
          ,000 ,
         8,    ,
 text:    ,gost, 18hHello, Madlen!'214''231'
          ,end ,
```

**Syntax note:** The string is `18h` (18 characters) here because "Hello, Madlen!" is two
characters shorter than "Hello, Assembler!".

**Compile and run:**

```sh
besmc hello.madlen
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
 PROGRAM           MADLEN-3.5
 001L               PROGRAM :  , NAME,
 0000 0002                     , *64 ,INFO
 0001                          , *74 ,
 0004               TEXT    :  , GOST,18HHELLO, MADLEN!'214''231'
 +++ CTPYKTYPA ПPOГPAMMЫ: +++
 +          KOMAHДЫ 00004              ИHCTPYKЦИИ 00009+
 ЧИCЛO ПEPФ.     09      ЧИCЛO OШИБ. OПEPATOPOB  00
...
 ДЛИHA БИБЛИOTEKИ  001 01
```

**Program output:**

```text
HELLO, MADLEN!
```

---

## Assembler БЕМШ — `hello.bemsh`

БЕМШ (BEMSH) is the BESM-6 macro-assembler developed at the Keldysh Institute in Moscow
(version 06/78). Unlike Madlen, **the entire source is written in Cyrillic** — both
instruction names and labels.

```text
ввд$$$
main    старт   512
        э64     инфо
        э74
инфо    мода    текст
        мода    текст
        конк    к'000000000'
        конк    к'100000000'
текст   текст   п'Hello, BEMSH!'
        конд    м40b'231'
        финиш
квч$$$
трн$$$
0-0
блмак
бтмалф
кнц$$$
```

**Syntax notes:**

- The file is divided into **sections** separated by markers ending in `$$$`:
  `ввд$$$` (input section), `квч$$$` / `трн$$$` / `кнц$$$` (auxiliary sections).
- `старт 512` declares the program entry with a 512-word stack.
- `э64` / `э74` are the BESM-6 print and newline I/O opcodes (same as Madlen's `*64` / `*74`).
- `п'...'` is a text literal in the GOST encoding.
- `конд м40b'231'` terminates the output (octal `231` = line-feed control code).
- `финиш` (finish) halts the program.
- BEMSH programs use **`main`** as the overlay entry point (all other languages use `program`).

**Compile and run:**

```sh
besmc hello.bemsh
./hello.exe
```

**Compiler listing excerpt** (`hello.lst`):

```text
ИПM MAKPO-БEMШ BEP.06/78
      ВВД◇◇◇
      ...
MAIN     HAM=01000   ДЛИHA MOДЛ=00001  ДЛИHA ПPOГ=00010 BXOДH=00001
ЧИCЛO OШИБOK=0000. MAKC CEPЬEЗH=0.
...
 ДЛИHA БИБЛИOTEKИ  001 01
```

`ЧИCЛO OШИБOK=0000` ("error count = 0") confirms the assembly succeeded.

**Program output:**

```text
HELLO, BEMSH!
```

---

## Mixed-language: Pascal calling Fortran — `caller.pascal` + `callee.ftn`

BESM-6 allowed programs to be written in multiple languages and linked together. Here a
Pascal main program calls a subroutine written in Fortran. Each file is compiled
independently to an object file (`.obj`), and then both objects are linked into one
executable.

**Main program** (`caller.pascal`):

```pascal
program main (output);
procedure hello; fortran;
_(
    hello;
_).
```

**Syntax note:** `procedure hello; fortran;` declares that the procedure `hello` is
implemented externally in Fortran. The Pascal compiler generates a call instruction but
does not compile the body itself.

**Fortran subroutine** (`callee.ftn`):

```fortran
        subroutine hello
        print 1000
 1000   format('Hello Fortran from Pascal!')
        end
```

**Compile and run (three steps):**

```sh
besmc -c caller.pascal          # Step 1: compile Pascal → caller.obj
besmc -c callee.ftn             # Step 2: compile Fortran → callee.obj
besmc caller.obj callee.obj     # Step 3: link both → caller.exe
./caller.exe
```

**Compiler listing excerpt** (`caller.lst`, from the link step):

```text
 LIBRARY OT  14/04/25 00.28.36   ← Pascal object loaded
 LIBRARY OT  14/04/25 00.28.36   ← Fortran object loaded
...
 ДЛИHA БИБЛИOTEKИ  004 03
```

The linked library is larger (`004 03`) than either language alone because it includes
both the Pascal and Fortran runtime libraries.

**Program output:**

```text
HELLO FORTRAN FROM PASCAL!
```
