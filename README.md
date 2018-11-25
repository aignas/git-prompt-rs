# Git prompt

This is a small CLI to display git info based on the current directory.

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

- [ ] Add docs
- [ ] Add support to specify color-scheme via the CLI parameters
- [ ] Add support to specify symbol-scheme via the CLI parameters
- [ ] Add benchmarks
- [ ] Add some e2e tests

## Contribute

Pull requests are welcome.
