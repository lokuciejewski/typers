# TypeRS, a terminal-based typing program in Rust
TypeRS is a simple, terminal-based utility for honing your typing skills. 

Currently supported sentence sources:
 - random Wikipedia page summary (`https://en.wikipedia.org/api/rest_v1/page/random/summary`)
 - custom sentence sources from text files

Planned support for sentence sources:
 - sentences from command line arguments
 - sentences from other programs (such as `fortune`)

## Installation
If you have `cargo` installed, use

`cargo install --git https://github.com/lokuciejewski/typers.git`


## Usage

The simplest way to get a one random Wikipedia article summary for typing is to use

`typers -w`

If you want to use a text file (currently supported are `json` and plain text file with sentences divided by newlines), use

`typers -f <path to file>`

Multiple sources can be combined along with `-n <number>` argument to specify the number of sentences. For example:

`typers -w -f test/test.json -n 10` 

will display 10 sentences selected randomly from either Wikipedia or file source.

`typers --help` will display a list of available options and commands.