# somafm-cli

A command line interface to interact with SomaFm. Play music from your terminal.

### prerequisites
* [mpv](https://mpv.io) - media player for the terminal

### development

running:
```
cargo run -- [command]
```

building:
```
cargo build
```

### help

```
USAGE:
    somafm-cli [FLAGS] [OPTIONS]

FLAGS:
    -c, --channels    Prints a list of SomaFm's channels
    -h, --help        Prints help information
    -V, --version     Prints version information

OPTIONS:
    -i, --info <channel>    Display the provided channel's information (TODO)
    -l, --last <channel>    Display the last song played from the provided channel
    -p, --play <channel>    Play a channel using mpv command
```