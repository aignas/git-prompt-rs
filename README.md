# Git prompt

[![Build Status](https://travis-ci.org/aignas/git-prompt-rs.svg?branch=master)](https://travis-ci.org/aignas/git-prompt-rs)
[![dependency status](https://deps.rs/repo/github/aignas/git-prompt-rs/status.svg)](https://deps.rs/repo/github/aignas/git-prompt-rs)

This is a small CLI to display git info based on the current directory.

<a href="https://asciinema.org/a/RlvQkQ57HZ6Pcw7pNlvuLAfjd" target="_blank"><img src="https://asciinema.org/a/RlvQkQ57HZ6Pcw7pNlvuLAfjd.svg" width="600"/></a>

## Features

- When counting the commit difference between the remote and the local clone it
  will default to the `master` on the remote if a branch with the same name is
  not found.  The default is customizable.

- It will make sure that the last character of the prompt is a space.  Some
  shells break because of this.

Example output:

<a href="https://asciinema.org/a/Vv45iWaTReTofmmqQFxT0XBnu" target="_blank"><img src="https://asciinema.org/a/Vv45iWaTReTofmmqQFxT0XBnu.svg" width="550"/></a>

## Install

### ArchLinux

```sh
$ git clone https://aur.archlinux.org/git-prompt-rs.git ${HOME}/aur/git-prompt-rs-git
$ cd ${HOME}/aur/git-prompt-rs-git
$ makepkg -si
```

### Mac

```sh
$ brew tap aignas/git-prompt-rs https://github.com/aignas/git-prompt-rs.git
$ brew install git-prompt-bin
```

### Using `cargo`

TODO @aignas (2019-06-11): This is planned, but not yet done

```sh
$ cargo install git-prompt-rs
```

## Build

For the time being build with cargo:

```bash
$ mkdir -p ${HOME}/bin # Ensure that this is in your path
$ cd .../git_prompt
$ cargo -Z unstable-options build --release --out-dir "${HOME}/bin"
```

## Setup

### ZSH

More info can be found at [ZSH documentation prompt expansion section](http://zsh.sourceforge.net/Doc/Release/Prompt-Expansion.html).

```
precmd() {
  local git_info=<path to the executable>
  export PS1="%F{blue}%~%f $(exec ${git_info})
%F{magenta}❯%f "
}
```

## Contribute

Pull requests are welcome.
