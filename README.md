# Git prompt

This is a small CLI to display git info based on the current directory.

<a href="https://asciinema.org/a/RlvQkQ57HZ6Pcw7pNlvuLAfjd" target="_blank"><img src="https://asciinema.org/a/RlvQkQ57HZ6Pcw7pNlvuLAfjd.svg" width="600"/></a>


## Features

- When counting the commit difference between the remote and the local clone it
  will default to the `master` on the remote if a branch with the same name is
  not found.  The default is customizable.

- It will make sure that the last character of the prompt is a space.  Some
  shells break because of this.

## Build

For the time being build with cargo:

```sh
$ mkdir -p ${HOME}/bin # Ensure that this is in your path
$ cd .../git_prompt
$ cargo build --release
$ cp ./target/release/git_prompt ${HOME}/bin
```

## Command-line options

```
git_prompt v0.1
aignas@github
Prints your git prompt info fast!

USAGE:
    git_prompt [FLAGS] [OPTIONS] [PATH]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --default-branch <default_branch>    default branch to use when printing diff status [env:
                                             GIT_PROMPT_DEFAULT_BRANCH=]  [default: master]

ARGS:
    <PATH>    Optional path to use for getting git info [default: .]
```

## Setup

### ZSH

More info can be found at <http://zsh.sourceforge.net/Doc/Release/Prompt-Expansion.html>.

```
precmd() {
  local git_info=<path to the executable>
  export PS1="%F{blue}%~%f ${git_info}
%F{magenta}❯%f "
}
```

## TODO

This is still work in progress.

- [ ] Add more screenshots with different outputs
- [ ] Add more docs
- [ ] Add support to specify color-scheme via the CLI parameters
- [ ] Add support to specify symbol-scheme via the CLI parameters
- [ ] Add tests with a mock git repo
- [ ] Add a `diverged` state for the ahead/behind status as described:
     <https://brson.github.io/2017/04/05/minimally-nice-maintainer>

## Contribute

Pull requests are welcome.
