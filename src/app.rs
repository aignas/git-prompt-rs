use clap::{App, Arg};

pub fn build() -> App<'static, 'static> {
    App::new("git-prompt")
        .version("v0.2.1")
        .author("aignas@github")
        .about("Prints your git prompt info fast!")
        .arg(
            Arg::with_name("PATH")
                .help("Optional path to use for getting git info")
                .index(1)
                .default_value("."),
        )
        .arg(
            Arg::with_name("default_branch")
                .short("d")
                .long("default-branch")
                .help("default branch to use when printing diff status")
                .env("GIT_PROMPT_DEFAULT_BRANCH")
                .default_value("master"),
        )
        .arg(
            Arg::with_name("print_updates")
                .long("print-updates")
                .help("Print the updates to the prompt as they happen.  This will at most print 3 lines of text which is useful for asynchronous updating when using in ZSH with zle -F or similar.")
            )
        .arg(
            Arg::with_name("status_symbols")
                .long("status-symbols")
                .help("status symbols to be used for the output.  The format is 'ok|staged|unmerged|unstaged|untracked'")
                .default_value("✔|●|✖|✚|…"),
        )
        .arg(
            Arg::with_name("branch_symbols")
                .long("branch-symbols")
                .help("branch symbols to be used for the output.  The format is 'ahead|behind'")
                .default_value("↑|↓"),
        )
        .arg(
            Arg::with_name("colorscheme")
                .long("colorscheme")
                .help("colorscheme to be used.  Either a preset or comma-separated byte values.")
                .default_value("simple"),
        )
        .arg(
            Arg::with_name("examples")
                .short("x")
                .help("print example output"),
        )
}
