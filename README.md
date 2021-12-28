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

## TODO

* add support for double vinculums
* add support for [fractions](https://en.wikipedia.org/wiki/Roman_numerals#Vinculum)

