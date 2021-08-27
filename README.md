# Colorer

**Colorer** is a simple text parsing command line utility that provides color to commands that do not have it by default. It works with Linux as well as MacOS.

## Commands

Not all commands are supported yet. If you implement a parser for a command or find an error please submit a pull request :).

Supported commands:
- `df`
- `dig`
- `docker` (still WIP)
- `env`
- `free`
- `last` and `lastb`
- `ls`
- `lsns`
- `nmap`
- `nslookup`
- `ping`


## Note on aliases
While most commands can be easily aliased, some may misbehave. An example is given by `ls`, which is implemented by taking into account the `-l` (list) flag.

## Configuration
Copy and paste the following block in you dotfile configuration.

``` text
alias df="clrr df"
alias dig="clrr dig"
alias docker="clrr docker"
alias env="clrr env"
alias free="clrr free"
alias last="clrr last"
alias lastb="clrr lastb"
alias ll="clrr ls -lahF"
alias lsns="clrr lsns"
alias nmap="clrr nmap"
alias nslookup="clrr nslookup"
alias ping="clrr ping"
```

## Screenshots

![df screenshot](screenshots/df.png)

![ping screenshot](screenshots/ping.png)

![nmap screenshot](screenshots/nmap.png)
