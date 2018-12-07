#![feature(test)]

extern crate clap;
use ansi_term::Color;
use clap::{App, Arg};
mod model;
mod view;

extern crate test;

fn main() {
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
            Arg::with_name("PATH")
                .help("Optional path to use for getting git info")
                .index(1)
                .default_value("."),
        )
        .get_matches();

    let c = view::Colors {
        default: Some(Color::Fixed(7)),
        ok: Some(Color::Green),
        high: Some(Color::Red),
        normal: Some(Color::Yellow),
        low: Some(Color::Fixed(252)),
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
    let path = matches.value_of("PATH").unwrap();

    let out = match git2::Repository::discover(path)
        .or_else(|e| Err(format!("{:?}", e)))
        .and_then(|r| prompt(&r, "master"))
    {
        Ok(p) => view::print(p, c, bs, ss),
        Err(_) => String::from(" "),
    };
    print!("{}", out)
}

pub fn prompt<T: model::Repo>(repo: &T, default: &str) -> model::R<model::Prompt> {
    let r = model::repo_status(repo)?;
    let b = r
        .branch
        .as_ref()
        .and_then(|b| model::branch_status(repo, b, default).ok());
    Ok(model::Prompt {
        repo: r,
        branch: b,
        local: model::local_status(repo)?,
    })
}

#[cfg(test)]
mod bench_main {
    use super::*;
    use crate::test::Bencher;

    #[bench]
    fn bench_discovery(b: &mut Bencher) {
        b.iter(|| git2::Repository::discover("."));
    }

    #[bench]
    fn bench_reading(b: &mut Bencher) {
        let r = git2::Repository::discover(".");
        b.iter(|| {
            r.as_ref()
                .or_else(|e| Err(format!("{:?}", e)))
                .and_then(|r| prompt(r, "master"))
        });
    }

    #[bench]
    fn bench_view(b: &mut Bencher) {
        b.iter(|| {
            let c = view::Colors {
                default: Some(Color::Fixed(7)),
                ok: Some(Color::Green),
                high: Some(Color::Red),
                normal: Some(Color::Yellow),
                low: Some(Color::Fixed(252)),
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
            format!("{}", view::print(p, c, bs, ss))
        });
    }
}
