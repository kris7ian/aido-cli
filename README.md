# Installation

### Simple Installation
```curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/kris7ian/aido-cli/main/install.sh | sh```

### Compile from source
You need to have rust installed to build it from source (https://www.rust-lang.org/tools/install).

```
# clone this repository
git clone .
cd aido-cli
# compile
cargo build --release
mkdir ~/.aido
cp target/release/aido ~/.aido
```

And then add `~/.aido` it to your PATH.

# Usage

### Generate commands
To generate a command describe what you want to do, to get better results don't write it as a question.
 E.g.:
 `aido move all files ending with .jpg to a folder called pictures`

 To copy the command to the clipboard use the `-c/--clipboard` flag
 E.g.:
  `aido -c list all docker containers`
  `aido --clipboard list all docker containers`


  ### Explain commands
  You can also let the AI explain a command for you.
  E.g.:
  `aido --explain "docker ps -a"`
  It's best to wrap the command in quotation marks.
