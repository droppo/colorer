% CLRR(1) Version 1.0.0 | User Commands

# NAME
**colorer** - Add color to commands that do not have it by default 

# SYNOPSIS
**colorer** [**OPTIONS**] command [**ARGS**]...

# DESCRIPTION
This is a simple parser useful highlight the output of some commands.

## Options
--help
: Print help and exit.

--version
: Print version and exit

# FILES
Colorer parses the output of a command thanks to some rules defined in TOML files. The program searches for these files in the directory `~/.config/colorer`.

# BUGS
See GitHub issues: <https://github.com/droppo/colorer/issues>

