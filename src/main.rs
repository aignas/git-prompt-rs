#![feature(test)]

extern crate clap;
use clap::{App, Arg};
mod examples;
mod model;
mod parse;
mod view;

extern crate test;

fn main() {
    if let Err(_) = run() {
        println!(); // print an empty line in case of an error
    };
}

fn run() -> model::R<()> {
    let matches = App::new("git_prompt")
        .version("v0.1")
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
        .get_matches();

    let c = matches
        .value_of("colorscheme")
        .ok_or("no value".to_string())
        .and_then(parse::colors)?;
    let bs = matches
        .value_of("branch_symbols")
        .ok_or("no value".to_string())
        .and_then(parse::bs)?;
    let ss = matches
        .value_of("status_symbols")
        .ok_or("no value".to_string())
        .and_then(parse::ss)?;
    let default_branch = &matches
        .value_of("default_branch")
        .ok_or("no value".to_string())?;

    if matches.is_present("examples") {
        print!("{}", examples::all().with_style(&c, &bs, &ss));
        return Ok(());
    }

    let repo = matches
        .value_of("PATH")
        .ok_or_else(|| "Unknown path".to_string())
        .and_then(|p| git2::Repository::discover(p).or_else(|e| Err(format!("{:?}", e))))?;
    let r = model::repo_status(&repo)?;
    let prompt = view::Prompt::new(&r).with_style(&c, &bs, &ss);

    if matches.is_present("print_updates") {
        let current = format!("{}", prompt);
        println!("{}", current);
        let prompt = prompt.with_branch(
            r.branch
                .as_ref()
                .and_then(|b| model::branch_status(&repo, b, default_branch).ok()),
        );
        let next = format!("{}", prompt);
        if next != current {
            let current = next;
            println!("{}", current);
        }
        let next = prompt
            .with_local(Some(model::local_status(&repo)))
            .to_string();
        if next != current {
            let current = next;
            println!("{}", current);
        }
    } else {
        println!(
            "{}",
            prompt
                .with_branch(
                    r.branch
                        .as_ref()
                        .and_then(|b| model::branch_status(&repo, b, default_branch).ok()),
                )
                .with_local(Some(model::local_status(&repo)))
        );
    }
    Ok(())
}

#[cfg(test)]
mod bench_main {
    use super::*;
    use crate::test::Bencher;
    use ansi_term::Color;

    #[bench]
    fn bench_discovery(b: &mut Bencher) {
        b.iter(|| git2::Repository::discover("."));
    }

    #[bench]
    fn bench_view(b: &mut Bencher) {
        b.iter(|| {
            let c = view::Colors {
                ok: Some(Color::Green),
                high: Some(Color::Red),
                normal: Some(Color::Yellow),
            };

            let ss = view::StatusSymbols {
                nothing: "✔",
                staged: "●",
                unmerged: "✖",
                unstaged: "✚",
                untracked: "…",
            };

            let bs = view::BranchSymbols {
                ahead: "↑",
                behind: "↓",
            };
            view::Prompt::new(&model::RepoStatus {
                branch: Some(String::from("master")),
                state: git2::RepositoryState::Clean,
            })
            .with_branch(Some(model::BranchStatus {
                ahead: 1,
                behind: 4,
            }))
            .with_local(Some(model::LocalStatus {
                staged: 0,
                unmerged: 0,
                unstaged: 0,
                untracked: 0,
            }))
            .with_style(&c, &bs, &ss)
            .to_string()
        });
    }
}
