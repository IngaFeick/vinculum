# Vinculum
A converter for roman numerals in [vinculum syntax](https://en.wikipedia.org/wiki/Roman_numerals#Vinculum) to arabic numbers, and vice versa.
Can be used as a library or as a command line tool.

## Usage

### Library

`vinculum::arabic2vinculum(4711)`

or

`vinculum::vinculum2arabic("your roman numeral goes here")`


### Command line

From arabic to vinculum:

```
$ cargo run 4711

I̅V̅DCCXI
```
From vinculum to arabic:

```
$ cargo run I̅V̅DCCXI

4711
```

## Number mappings

Normal roman numerals:
|Numeral|Arabic|
|-------|------|
|I | 1 |
|V | 5 |
|X | 10 |
|L | 50 |
|C | 100 |
|D | 500 |
|M | 1000 |

Single vinculum `I̅ V̅ X̅ L̅ C̅ D̅ M̅`:

(Depending on the font the vinculums in this table might be misaligned)
|Numeral|Arabic|Power of ten|
|-------|------|------------|
|I̅ | 1000 |10^3|
|V̅ | 5000 ||
|X̅ | 10.000 |10^4|
|L̅ | 50.000 ||
|C̅ | 100.000 |10^5|
|D̅ | 500.000 ||
|M̅ | 1.000.000 |10^6|


Double vinculum `I̿ V̿ X̿ L̿ C̿ D̿ M̿ `:
(Depending on the font the vinculums in this table might be misaligned)
|Numeral|Arabic|Power of ten|
|-------|------|------------|
|I̿ | 1.000.000 |10^6|
|V̿ | 5.000.000 ||
|X̿ | 10.000.000 |10^7|
|L̿ | 50.000.000 ||
|C̿ | 100.000.000 |10^8|
|D̿ | 500.000.000 ||
|M̿ | 1.000.000.000 |10^9|

In lieu of a specification, numbers larger than 10^9 are represented by completely fictional glyphs comprised of combinations of double vinculums with single/double over- and underlines and single/double vertical centered strokes. (PRs with better ideas on how to represent this are _very_ welcome)

Examples:

```
Double vinculum with single underline (10^12):

I̲̿ V̲̿ X̲̿ L̲̿ C̲̿ D̲̿ M̲̿

Double vinculum with double underline (10^15):

I̳̿ V̳̿ X̳̿ L̳̿ C̳̿ D̳̿ M̳̿

Double vinculum with double underline and vertical middle bar (10^18):

I⃒̳̿ V⃒̳̿ X⃒̳̿ L⃒̳̿ C⃒̳̿ D⃒̳̿ M⃒̳̿

Double vinculum with double underline and double vertical middle bar (10^21):

I⃦̳̿ V⃦̳̿ X⃦̳̿ L⃦̳̿ C⃦̳̿ D⃦̳̿ M⃦̳̿
```
The readability of these grapheme clusters strongly depends on the font. `Menlo` for example does a good job of placing the combining characters at the right positions. In other fonts, this tool doubles as an involuntarily Zalgo text generator.


## TODO

* in some fonts the 20D2 code point (single middle vertical stroke) generates some white space to the right of the grapheme. Replace with a better symbol.
* add support for [fractions](https://en.wikipedia.org/wiki/Roman_numerals#Vinculum)

