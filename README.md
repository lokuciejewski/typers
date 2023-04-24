# TypeRS, a terminal-based typing program in Rust

TypeRS is a simple, terminal-based utility for honing your typing skills.

Currently supported sentence sources:

- random Wikipedia page summary (`https://en.wikipedia.org/api/rest_v1/page/random/summary`)
- custom sentence sources from text files
- pipe sentences from other programs (such as `fortune`)

Planned support for sentence sources:

- sentences from command line arguments

Planned features:

- Add local high score table
- Keep local statistics and display changes in accuracy/WPMs

## Installation

Install `cargo` and use

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

Sentences can be piped into `typers` using `|` operator:

`echo "This is a test sentence" | typers`

will enable user to type the piped sentence. The sentences are divided by `.` character. For example:

`echo "First sentence. Second sentence." | typers`

will choose one of the two sentences at random.

The pipe operator can be combined with another command line arguments to get a more variable sentence sources:

`echo "This is a test sentence." | typers -w -n 10`

will produce 10 random sentences from either the piped input or Wikipedia source.
