# Colorer

![Crates.io](https://img.shields.io/crates/v/colorer)
![GitHub last commit](https://img.shields.io/github/last-commit/droppo/colorer)

**Colorer** is a simple and extensible text parsing command line utility that is intended to add color to commands that do not have it by default.

The command uses user's defined configuration files.

## Installation

### Installing using Cargo

If Rust toolchain is installed, it is possible to install **colorer** by using `cargo`:

``` bash
cargo install colorer # from crates.io
```

or, after cloning this repository, run:
```bash
cargo install --path .
```

### Installing from releases

Another way to intall this program is to download the binary from the release page. There are multiple choices:  
- download the binary and put it in the path  
- download the package (deb or rpm) and install it. This is the suggested method because it also creates the directory containing the configuration files.


## Usage

The program searches for configuration files in `$HOME/.config/colorer`. This repository contains some basic configuration files available in `color_patters` directory.

To use **colorer**, pass the command to be runned as argument, for example:
```bash
colorer df -h
```

## Define new configuration files

The files used to define a color scheme for a command are simple TOML files named after the commads (e.g. `ls.toml`). A color scheme contains a `command` vector in which every component contains the following variables:  
- `regex`: the regex used to find the text. This value is mandatory.  
- `default decorator`: it contains an array of colors. An array is used so it is possible to add a background color, a foreground color as well as a bold, italics. Also this value is mandatory.  
- `optionals_decorators`: this value is optional and contains an array of alternative values for the string found with the regex. The elements are represented as a tuple where the first value contains a regex that searches for a pattern **only on what has been found by the mandatory regex**, and a list of decorators.  


An example is given by the following snippet defined for `df`. This is used to highlight the usage represented as percentage.
``` toml
[[command]]
regex = "(?<=(\\b(\\d+((\\.|\\,)\\d+)?(\\w+)?\\s+){3}))\\d+%"   # main regex.
                                                                # This searches for all
                                                                # percentages
default_decorator = ["GreenFg"] # this is the default decorator and it is used 
                                # if none of the optional decorators are used.
                                # By exclusion, for this example, it includes 
                                # all the values from 0% up to 49%
optional_decorators = [
    # the following regexs apply only for the text extracted by the main regex
    [
        "([5-7][0-9])%", # values from 50% up to 79%
        ["YellowFg"] # yellow text
    ], 
    [
        "([8][0-9]|[9][0-6])%", # values from 80% up to 96%
        ["MagentaFg"] # magenta text
    ], 
    [
        "([9][7-9]|100)%", # values above 97%
        ["RedBg"] # red backgroud
    ]
]
```


It is possible to define color schemas for a command (`df`, `ping`, `nmap`) as well a for sub-commands (`docker ps`, `docker search`). For sub-commands definitions, replace `command` with `subcommand.<subcommand>`. For example, in case of `docker search`:

``` toml
[[subcommand.search]]
regex = "(^[^NAME]\\w+(?=\\s)|(?<=(^\\w+\\/))\\S+)"
default_decorator = ["YellowFgBright", "Bold"]
```


## Notes

It is always a good idea to use double-dash if **colorer** is used to alias a command.
For example, by defining `ll='colorer ls -lahF'`, `ll --help` will print the help message from colorer. This problem can be avoided with `ll='colorer -- ls -lahF'`

## Screenshots

![df screenshot](screenshots/df.png)

![ping screenshot](screenshots/ping.png)

![nmap screenshot](screenshots/nmap.png)
