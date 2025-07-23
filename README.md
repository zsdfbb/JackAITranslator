# JackTranslate

AI-based command-line translation tool

## Install

1. compile and install:

```bash
cd aitrans
cargo build --release

cd ..
./install.sh
```

2. install binary

```bash
./install.sh --bin
```


## Usage

```text
AITrans - A command-line tool for translating text between languages.

USAGE:
  aitrans --from [input language] --to [output language] [INPUT]
  aitrans [INPUT]           // chinese to english
  aitrans -r [INPUT]        // english to chinese
  aitrans -l, --list        // list all supported languages
  aitrans -v, --version     // Prints version information
  aitrans --debug [INPUT]   // Prints debug information
```
## Example
<img width="1197" height="97" alt="image" src="https://github.com/user-attachments/assets/7ec9b22a-3eeb-44d3-9669-e90a4185380e" />
