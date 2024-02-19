# minbox

Minbox is a simple, minimalistic command-line tool written in Rust, inspired by BusyBox. The only reason I'm developing this project is to learn Rust programming.

## Batteries Included

Available commands:

```bash
del [files...]  # Delete files and/or folders.
dir             # Print the current working directory.
dt [format]     # Get date and time with optional format.
f               # List files and folder with some metadata.
md <name>       # Create folder with specified name.
mf <name>       # Create file with specified name.
r <name> [--with_lines, -l] [--from <number>, -f <number>] [--to, -t]
                # Read file with optional line numbers and range.
say [string...] # Echo strings in console.
```

## Getting Started

To get started with minbox, you need `qrepo`, `rustc`, and `cargo` installed on your system. Then, you can type the following on your command line:

```bash
git clone https://github.com/nthnn/minbox.git --depth 1
cd minbox
qrepo run build
```

This will build the minbox from source files. If you want to install it to your system binary path, type the following:

```bash
qrepo run install
```

## License

Ladivic is distributed under the [GNU General Public License v3.0](LICENSE). For further details, refer to the LICENSE file.