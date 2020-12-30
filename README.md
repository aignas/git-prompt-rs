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

## Tests with large repos

One of the larger repos I could find on the internet is for [Chromium](https://github.com/chromium/chromium) and I did shallow clone the repo on my local machine by:
```shell
$ git clone -b master git@github.com:chromium/chromium.git --depth=1
```

Some stats about the repo:
```shell
$ git ls-files | wc -c
27032106

```

Then I ran my benchmarks with the chromium repo (ThinkPad T420s with an SSD):
```
$ GIT_PROMPT_BENCH_PATH=~/src/github/chromium cargo bench
branch_status           time:   [119.97 us 120.68 us 121.52 us]
repo_status             time:   [21.080 us 21.137 us 21.212 us]
local_status            time:   [2.3654 s 2.3905 s 2.4161 s] # This is even more than doing git status from the shell!
```

This shows that the local_status is indeed the major offender here, which is
expected and documented well in the README in
[romkatv/gitstatus](https://github.com/romkatv/gitstatus).  The thing is that git status is always going to be the bottleneck in case of larger repositories, because the CLI needs to scan through all committed files and the best way to optimize this path is to:
1. do less kernel calls when scanning.
2. do less scanning.

### Do less kernel calls when scanning

We could employ a server/client architecture in order to only do the necessary scanning. Possible ways to improve this:
* Subscribe to fs events at the root of the repository and then do a git status on file event changes.
  * Drawbacks:
    * During builds, lots of files may be created, which affects the performance.
    * Reimplementing watchman?
      * https://facebook.github.io/watchman/
      * https://github.blog/2018-04-05-git-217-released/#speeding-up-status-with-watchman
      * Or at least we could integrate our git status with watchman, like: https://github.com/jgavris/rs-git-fsmonitor
  * Benefits:
    * Fast incremental updates of the in-memory status.

### Just use git status

After a bit of thought, I realized, that git status is the solution I am looking for. It is fast and it works well with any other tooling that I may use:
* Git VFS
* `watchman`

And the best of all, it works with large repos and I do not need to maintain code.  The only thing I do need to do is to setup the git hooks to ensure that the watchman is used.
