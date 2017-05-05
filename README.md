
# scotext
Command line app that scores input according to how likely it contains english text.

### Installation

If you've not already done so, install rust:
https://www.rust-lang.org/

Then install via cargo using:
```bash
$ cargo install scotext
```

### Help

```bash
$ scotext -h
scotext 0.1.0
Gavyn Riebau
Scores input based on english language character frequency

USAGE:
    scotext [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Increases the verbosity of output

OPTIONS:
    -w, --dictionary <dictionary>    Path to a dictionary file with english language words. If one of the words in the dictionary is found in the input, the text score will be increased.
```

### Examples

Score some plaint text using a dictionary file:
```bash
$ cat plaintext
In another space entirely, it was early morning in Ankh-Morpork, oldest and greatest and grubbiest of cities.
$ cat plaintext | scotext -w ~/words.txt 
 1447.86
```

Score some encrypted text using a dictionary file:
```bash
$ cat encrypted 
x_P_^EYTCBAPRTT_EXCT]HXEFPBTPC]H\^C_X_VX_p_ZY|^CA^CZ^]UTBEP_UVCTPETBEP_UVCDSSXTBE^WRXEXTB
$ cat encrypted | scotext -w ~/words.txt 
  607.39
```


