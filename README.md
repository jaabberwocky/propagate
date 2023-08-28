# Propagate: Synthetic Data Generation CLI Tool

Propagate is written in Rust for generating synthetic data. 

## Installation

To use this tool, you need to have Rust installed on your system. You can install Rust by following the instructions provided on the official Rust website (https://www.rust-lang.org/tools/install).

Once Rust is installed, you can clone this repository and compile the code:

```bash
git clone https://github.com/jaabberwocky/propagate.git
cd propagate
cargo build --release

Usage

To generate synthetic data using Propagate, you can run the following command:

Markdown
propagate --columns column1,column2,column3 --rows 100 --output output.csv


Where:

--columns or -c specifies the column names to generate. Separate multiple column names with commas.
--rows or -r specifies the number of rows to generate. The default value is 50.
--output or -o specifies the name of the output file. The default value is "data.csv".

For example, the command above will generate 100 rows of synthetic data with the column names "column1", "column2", and "column3". The generated data will be saved to the "output.csv" file.

Output

The generated column names will be printed to the console.

Author

This tool was created by Tobias (tobias@tobias.dev).

License

This tool is open-source and released under the MIT License. See the LICENSE file for more information.

```