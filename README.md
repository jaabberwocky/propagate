# Propagate: Data Generation CLI Tool

```
    =====================================================
    PPPP   RRRR    OOO   PPPP   A   GGGG   A   TTTTT EEEE
    P   P  R   R  O   O  P   P A A  G     A A    T   E
    PPPP   RRRR   O   O  PPPP  AAAA G  GG AAAA   T   EEEE
    P      R  R   O   O  P    A   A G   G A   A  T   E
    P      R   R   OOO   P    A   A  GGG  A   A  T   EEEE
    =====================================================
```

Propagate is a simple data generator tool written (lovingly) in Rust!

## Installation

To use this tool, you can either:

1. Download the latest [release](https://github.com/jaabberwocky/propagate/releases/latest) *or*
2. Compile from source

You need to have Rust installed on your system. You can install Rust by following the instructions provided on the official Rust website (https://www.rust-lang.org/tools/install).

Once Rust is installed, you can clone this repository and compile the code:

```bash
git clone https://github.com/jaabberwocky/propagate.git
cd propagate
cargo build --release
```

## Usage
```
Usage: propagate [OPTIONS] --columns <COLUMNS> --ranges <RANGES>

Options:
  -c, --columns <COLUMNS>    Column names to generate
  -r, --ranges <RANGES>      Ranges for each column separated by comma (e.g. 0:100,5:25).
                             If ranges provided are less than columns, the first range will be used for all columns.
                             Panics if ranges provided are more than columns.
  -n, --num-rows <NUM_ROWS>  Number of rows to generate [default: 50]
  -o, --output <OUTPUT>      Output file name [default: data.csv]
  -s, --sep <SEP>            Seperator for output file [default: ", "]
  -h, --help                 Print help
  -V, --version              Print version
```

Columns and ranges ***must*** be provided and comma-separated. To ensure consistent string parsing, enclose them in quotes (e.g. "v1, v2, v3").
## Examples:

Generates a file with 50 rows and 3 columns (v1, v2, v3) with values between 1 and 100, 2 and 50, and -100 and 100 respectively. The file is saved to "data.csv" (default) and the values are comma separated (default).

```bash
propagate -c "v1, v2, v3" -r "1:100, 2:50, -100:100"
```

Generates a file with 100 rows and 3 columns (v1, v2, v3) with values all between 1 and 100. The output file is named "data.csv" and the values are pipe separated (e.g. "|").

```bash
propagate -c "v1, v2, v3" -r "1:100" -n 100 -o "data.csv" -s "|"
```
