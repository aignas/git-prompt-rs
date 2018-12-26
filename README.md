# Git prompt

[![Build Status](https://travis-ci.org/aignas/git_prompt.svg?branch=master)](https://travis-ci.org/aignas/git_prompt)

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

## Build

For the time being build with cargo:

```bash
$ mkdir -p ${HOME}/bin # Ensure that this is in your path
$ cd .../git_prompt
$ cargo -Z unstable-options build --release --out-dir "${HOME}/bin"
```

## Command-line options

```
git_prompt v0.1
aignas@github
Prints your git prompt info fast!

USAGE:
    git_prompt [FLAGS] [OPTIONS] [PATH]

FLAGS:
    -x               print example output
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --branch-symbols <branch_symbols>    branch symbols to be used for the output.  The format is 'ahead|behind'
                                             [default: ↑|↓]
        --colorscheme <colorscheme>          colorscheme to be used.  Either a preset or comma-separated byte values.
                                             [default: simple]
    -d, --default-branch <default_branch>    default branch to use when printing diff status [env:
                                             GIT_PROMPT_DEFAULT_BRANCH=]  [default: master]
        --status-symbols <status_symbols>    status symbols to be used for the output.  The format is
                                             'ok|staged|unmerged|unstaged|untracked' [default: ✔|●|✖|✚|…]


ARGS:
    <PATH>    Optional path to use for getting git info [default: .]
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
