#!/bin/sh
# poooh@aikipooh.name

#baseurl=https://github.com/kris7ian/aido-cli/releases/tag/v0.5.4
baseurl=https://github.com/kris7ian/aido-cli/releases/latest/download

# aido-aarch64-apple-darwin.tar.gz
# aido-x86_64-apple-darwin.tar.gz
# aido-x86_64-unknown-linux-gnu.tar.gz
# aido-x86_64-pc-windows-gnu.zip

log() {
  echo $* >&2
}

bindir=~/.aido

export_cmd="export PATH=\$PATH:$bindir"

# Adds to .bashrc/.zshrc
add_to_shrc() {
  for rc in .bashrc .zshrc; do
    test -f ~/$rc && {
    log "  Updating PATH in $rc"
      printf "\n$export_cmd" >> ~/$rc
    }
  done
}

case `uname -m` in
  x86_64) mach="x86_64";;
  arm|arm64) mach="aarch64";;
esac

case `uname -s` in
  Linux) os="unknown-linux-gnu";;
  Darwin) os="apple-darwin"
esac

fn=aido-$mach-$os

log "base fn: $fn"

url=$baseurl/${fn}.tar.gz
echo "url: $url"

if true; then
  if ! curl $url -sL; then
    log "Problem downloading $url. Please check whether your OS/mach is supported."
    exit 1
  fi | {
    log "Downloaded the archive file"

    mkdir -p $bindir # Won't object if it exists already
    cd $bindir
    tar xfz - 2>/dev/null || {
      log "Bad format"
      exit 1
    }
  }
fi || {
  log "There was a problem with the installation"
  exit 1
}

# Check we're already in the PATH
if ! echo $PATH|awk 'BEGIN{RS=":"}{print}'|grep -q $bindir$; then
  # Need to add to .bashrc or whatever
  add_to_shrc
fi

cat <<EOF

The installation has finished successfully. To use the program right now, pleasse run this command in your current shell session:
$export_cmd
EOF
