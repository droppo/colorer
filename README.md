# Colorer

**Colorer** is a simple and extensible text parsing command line utility with the goal af adding color to commands that do not have it by default.
The command uses user's defined configuration files.

![Crates.io](https://img.shields.io/crates/v/colorer)
![GitHub last commit](https://img.shields.io/github/last-commit/droppo/colorer)

## Installation

The most simple way is to install colorer is to by downloading the program from the release page and put it in your path.

Another way is to use `cargo`:

``` bash
cargo install colorer
```

or, after cloning this repository, run:
```bash
cargo install --path .
```

## Usage

The program searches for configuration files in ```$HOME/.config/colorer```. This repository contains some basic configuration files available in `color_patters` directory.

To use **colorer**, pass the command to be runned as argument, for example:
```bash
colorer df -h
```

## Definig new configuration files
The files used to define a color scheme for a command are simple TOML files. A color scheme contains a `command` vector in which every component contains the following variables:  
- `regex`: the regex used to find the text. This value is mandatory.  
- `default decorator`: it contains an array of colors. An array is used so it is possible to add a background color, a foreground color as well as a bold, italics. Also this value is mandatory.  
- `optionals_decorators`: this value is optional and contains an array of alternative values for the string found with the regex. The elements are represented as a tuple where the first value af the tuple contains a regex that searches for a pattern *using the text found by the mandatory regex*, and a list of decorators.  


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
