# somafm-cli

A command line interface to interact with SomaFm. Play music from your terminal ([mpv](https://mpv.io) is required)

This is a "learning rust" project so files are subject to massive change as I learn what I'm doing.

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