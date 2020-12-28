// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use clap::Clap;

use std::path::PathBuf;

/// Prints your git prompt info fast!
#[derive(Clap)]
#[clap(version = "0.3.0", author = "aignas@github", bin_name = "git-prompt")]
pub struct Opts {
    /// sets the path for which we print the results
    #[clap(default_value = ".")]
    pub path: PathBuf,

    /// Print example output and exit
    #[clap(short, long)]
    pub examples: bool,

    /// Print the updates to the prompt as they happen.  This will at most print 3 lines of text
    /// which is useful for asynchronous updating when using in ZSH with zle -F or similar.
    #[clap(short, long)]
    pub print_updates: bool,

    /// default_branch to use when printing diff status
    #[clap(
        short,
        long,
        default_value = "master",
        env = "GIT_PROMPT_DEFAULT_BRANCH"
    )]
    pub default_branch: String,

    /// status symbols to be used for the output. The format is
    /// 'ok|staged|unmerged|unstaged|untracked'.
    #[clap(long, default_value = "✔|●|✖|✚|…")]
    pub status_symbols: String,

    /// branch symbols to be used for the output. The format is 'ahead|behind'.
    #[clap(long, default_value = "↑|↓")]
    pub branch_symbols: String,

    /// default_branch to use when printing diff status
    #[clap(long, default_value = "simple")]
    pub colorscheme: String,
}
