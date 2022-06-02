% CLRR(1) Version 1.0.0 | User Commands

# NAME
**colorer** - Add color to commands that do not have it by default 

# SYNOPSIS
**colorer** [**OPTIONS**] command [**ARGS**]...

# DESCRIPTION
**Colorer** is a simple and extensible text parsing command line utility that is intended to add color to commands that do not have it by default.

The command uses user's defined configuration files.


## Options
--help
: Print help and exit.

--version
: Print version and exit

# FILES
Colorer parses the output of a command thanks to some rules defined in TOML files. The program searches for these files in the directory `/etc/colorer`.

# BUGS
See GitHub issues: <https://github.com/droppo/colorer/issues>

