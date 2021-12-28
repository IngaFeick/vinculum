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

Single vinculum (reaches up to a million):
|Numeral|Arabic|Power of ten|
|-------|------|------------|
|I̅ | 1000 |10^3|
|V̅ | 5000 ||
|X̅ | 10.000 |10^4|
|L̅ | 50.000 ||
|C̅ | 100.000 |10^5|
|D̅ | 500.000 ||
|M̅ | 1.000.000 |10^6|


Double vinculum (up to a billion):
|Numeral|Arabic|Power of ten|
|-------|------|------------|
|I̿ | 1.000.000 |10^6|
|V̿ | 5.000.000 ||
|X̿ | 10.000.000 |10^7|
|L̿ | 50.000.000 ||
|C̿ | 100.000.000 |10^8|
|D̿ | 500.000.000 ||
|M̿ | 1.000.000.000 |10^9|


## TODO

* add support for double vinculums
* add support for [fractions](https://en.wikipedia.org/wiki/Roman_numerals#Vinculum)

