#![feature(test)]

extern crate clap;
use clap::{App, Arg};
mod examples;
mod model;
mod parse;
mod view;

extern crate test;

fn main() {
    print!("{}", run().unwrap())
}

fn run() -> model::R<String> {
    let matches = App::new("git_prompt")
        .version("v0.1")
        .author("aignas@github")
        .about("Prints your git prompt info fast!")
        .arg(
            Arg::with_name("default_branch")
                .short("d")
                .long("default-branch")
                .help("default branch to use when printing diff status")
                .env("GIT_PROMPT_DEFAULT_BRANCH")
                .default_value("master"),
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
            Arg::with_name("no-branch")
                .long("no-branch")
                .help("don't print the branch")
            )
        .arg(
            Arg::with_name("no-diff")
                .long("no-diff")
                .help("don't print the diff")
            )
        .arg(
            Arg::with_name("no-status")
                .long("no-status")
                .help("don't print the status")
            )
        .arg(
            Arg::with_name("examples")
                .short("x")
                .help("print example output"),
        )
        .arg(
            Arg::with_name("PATH")
                .help("Optional path to use for getting git info")
                .index(1)
                .default_value("."),
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

    let v = if matches.is_present("examples") {
        format!("{}", examples::all().with_style(&c, &bs, &ss))
    } else {
        matches
            .value_of("PATH")
            .ok_or_else(|| "Unknown path".to_string())
            .and_then(|p| git2::Repository::discover(p).or_else(|e| Err(format!("{:?}", e))))
            .and_then(|repo| {
                let r = model::repo_status(&repo)?;
                let l = if matches.is_present("no-status") {
                    None
                } else {
                    Some(model::local_status(&repo)?)
                };
                let b = if matches.is_present("no-diff") {
                    None
                } else {
                    r.branch
                        .as_ref()
                        .and_then(|b| model::branch_status(&repo, b, "master").ok())
                };
                Ok(model::Prompt {
                    repo: if matches.is_present("no-branch") {
                        None
                    } else {
                        Some(r)
                    },
                    branch: b,
                    local: l,
                })
            })
            .map(|p| view::print(p, &c, &bs, &ss))
            .unwrap_or_else(|_| String::from(" "))
    };
    Ok(v)
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
            let p = model::Prompt {
                repo: model::RepoStatus {
                    branch: Some(String::from("master")),
                    state: git2::RepositoryState::Clean,
                },
                branch: Some(model::BranchStatus {
                    ahead: 1,
                    behind: 4,
                }),
                local: model::LocalStatus {
                    staged: 0,
                    unmerged: 0,
                    unstaged: 0,
                    untracked: 0,
                },
            };
            view::print(p, &c, &bs, &ss).to_string()
        });
    }
}
