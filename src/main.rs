#![feature(test)]

extern crate clap;
use clap::{App, Arg};
mod examples;
mod model;
mod parse;
mod view;

extern crate test;

fn main() {
    run().unwrap_or(());
}

fn run() -> model::R<()> {
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
            Arg::with_name("stream")
                .long("stream")
                .help("Print the stuff as a stream with updates, which is helpful when using in ZSH with zle -F.")
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

    if matches.is_present("examples") {
        print!("{}", examples::all().with_style(&c, &bs, &ss));
    } else if matches.is_present("stream") {
        let repo = matches
            .value_of("PATH")
            .ok_or_else(|| "Unknown path".to_string())
            .and_then(|p| git2::Repository::discover(p).or_else(|e| Err(format!("{:?}", e))));

        if let Err(_e) = repo {
            println!("");
            return Ok(());
        }
        let repo = repo.unwrap();

        let r = model::repo_status(&repo)?;
        let p = model::Prompt {
            repo: Some(r.clone()),
            branch: None,
            local: None,
        };
        let v = view::print(p, &c, &bs, &ss).to_string();
        if v != " " {
            println!("{}", v);
        }

        let b = r
            .branch
            .as_ref()
            .and_then(|b| model::branch_status(&repo, b, "master").ok());
        let p = model::Prompt {
            repo: Some(r.clone()),
            branch: b.clone(),
            local: None,
        };
        let n = view::print(p, &c, &bs, &ss).to_string();
        if v != n {
            println!("{}", n);
        }
        let v = n;

        let l = Some(model::local_status(&repo)?);
        let p = model::Prompt {
            repo: Some(r),
            branch: b,
            local: l,
        };
        let n = view::print(p, &c, &bs, &ss).to_string();
        if v != n {
            println!("{}", n);
        }
    } else {
        let v = matches
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
            .unwrap_or_else(|_| String::from(" "));
        println!("{}", v);
    };
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
            let p = model::Prompt {
                repo: Some(model::RepoStatus {
                    branch: Some(String::from("master")),
                    state: git2::RepositoryState::Clean,
                }),
                branch: Some(model::BranchStatus {
                    ahead: 1,
                    behind: 4,
                }),
                local: Some(model::LocalStatus {
                    staged: 0,
                    unmerged: 0,
                    unstaged: 0,
                    untracked: 0,
                }),
            };
            view::print(p, &c, &bs, &ss).to_string()
        });
    }
}
