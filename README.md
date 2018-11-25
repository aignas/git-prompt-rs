# Git prompt

This is a small CLI to display git info based on the current directory.

## Build

For the time being build with cargo:

```sh
$ mkdir -p ${HOME}/bin # Ensure that this is in your path
$ cd .../git_prompt
$ cargo build --release
$ cp ./target/release/git_prompt ${HOME}/bin
```

## Setup

For all options on how you can customize the output, run:
```sh
$ git_prompt --help
```

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

- [ ] Add more docs
- [ ] Add support to specify color-scheme via the CLI parameters
- [ ] Add support to specify symbol-scheme via the CLI parameters
- [ ] Add tests with a mock git repo

## Contribute

Pull requests are welcome.
