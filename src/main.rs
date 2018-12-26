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
        .arg(
            Arg::with_name("PATH")
                .help("Optional path to use for getting git info")
                .index(1)
                .default_value("."),
        )
        .get_matches();

    let c = parse_colors(matches.value_of("colorscheme").unwrap()).unwrap();
    let ss = parse_ss(matches.value_of("status_symbols").unwrap()).unwrap();
    let bs = parse_bs(matches.value_of("branch_symbols").unwrap()).unwrap();

    if matches.is_present("examples") {
        let master = Some("master");
        let clean = git2::RepositoryState::Clean;
        let rebase = git2::RepositoryState::Rebase;

        fn b(ahead: usize, behind: usize) -> Option<model::BranchStatus> {
            Some(model::BranchStatus { ahead, behind })
        }
        fn s(
            staged: usize,
            unstaged: usize,
            unmerged: usize,
            untracked: usize,
        ) -> model::LocalStatus {
            model::LocalStatus {
                staged,
                unstaged,
                unmerged,
                untracked,
            }
        }

        Examples::new()
            .add("new", None, clean, None, s(0, 3, 0, 0))
            .add("ok", master, clean, b(0, 0), s(0, 0, 0, 0))
            .add("stage", master, clean, b(0, 0), s(3, 0, 0, 0))
            .add("partial", master, clean, b(0, 0), s(3, 12, 0, 0))
            .add(
                "conflicts",
                Some("a83e2a3f"),
                rebase,
                b(0, 3),
                s(0, 2, 1, 0),
            )
            .add("rebase", master, rebase, b(0, 3), s(0, 3, 0, 0))
            .add("diverged", master, rebase, b(12, 3), s(0, 0, 0, 3))
            .print(&c, &bs, &ss);
        return;
    }

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

fn parse_colors(input: &str) -> model::R<view::Colors> {
    if input == "simple" {
        // Add colorscheme presets here
        return Ok(view::Colors {
            ok: Some(Color::Fixed(2)),
            high: Some(Color::Fixed(1)),
            normal: Some(Color::Fixed(3)),
        });
    }

    let parts: Vec<u8> = input
        .split(',')
        .map(|s| s.parse::<u8>().unwrap_or(0))
        .collect();

    match parts.len() {
        3 => Ok(view::Colors {
            ok: Some(Color::Fixed(parts[0])),
            high: Some(Color::Fixed(parts[1])),
            normal: Some(Color::Fixed(parts[2])),
        }),
        l => Err(format!(
            "Unknown custom color input: {}. Expected 4 terms, but got {}.",
            input, l
        )),
    }
}

fn parse_ss(input: &str) -> model::R<view::StatusSymbols> {
    let parts: Vec<&str> = input.split('|').collect();

    match parts.len() {
        5 => Ok(view::StatusSymbols {
            nothing: parts[0],
            staged: parts[1],
            unmerged: parts[2],
            unstaged: parts[3],
            untracked: parts[4],
        }),
        _ => Err(format!("Unknown input format: {}", input)),
    }
}

fn parse_bs(input: &str) -> model::R<view::BranchSymbols> {
    let parts: Vec<&str> = input.split('|').collect();

    match parts.len() {
        2 => Ok(view::BranchSymbols {
            ahead: parts[0],
            behind: parts[1],
        }),
        _ => Err(format!("Unknown input format: {}", input)),
    }
}

struct Examples {
    examples: std::collections::HashMap<String, model::Prompt>,
}

impl Examples {
    pub fn new() -> Examples {
        use std::collections::HashMap;
        Examples {
            examples: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        key: &str,
        br: Option<&str>,
        state: git2::RepositoryState,
        branch: Option<model::BranchStatus>,
        local: model::LocalStatus,
    ) -> &mut Examples {
        let repo = model::RepoStatus {
            branch: br.map(|s| s.to_owned()),
            state,
        };
        self.examples.insert(
            key.to_string(),
            model::Prompt {
                repo,
                branch,
                local,
            },
        );
        self
    }

    pub fn print(&self, c: &view::Colors, bs: &view::BranchSymbols, ss: &view::StatusSymbols) {
        let max_length = self
            .examples
            .keys()
            .map(|l| l.len())
            .max()
            .expect("failed to get the maximum example key length");
        for (l, p) in &self.examples {
            println!(
                "{0:>1$}: {2}",
                l,
                max_length,
                view::print(p.clone(), c.clone(), bs.clone(), ss.clone())
            );
        }
    }
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
            view::print(p, c, bs, ss).to_string()
        });
    }
}
