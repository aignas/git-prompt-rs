#![feature(test)]

extern crate clap;
mod app;
mod examples;
mod model;
mod parse;
mod view;

extern crate test;

fn main() {
    if run().is_err() {
        println!(); // print an empty line in case of an error
    };
}

fn run() -> model::R<()> {
    let matches = app::build().get_matches_from(std::env::args());
    let c = matches
        .value_of("colorscheme")
        .ok_or_else(|| "BUG: colorscheme has no default".to_owned())
        .and_then(parse::colors)?;
    let bs = matches
        .value_of("branch_symbols")
        .ok_or_else(|| "BUG: branch_symbols has no default".to_owned())
        .and_then(parse::bs)?;
    let ss = matches
        .value_of("status_symbols")
        .ok_or_else(|| "BUG: status_symbols has no default".to_owned())
        .and_then(parse::ss)?;
    let default_branch = matches
        .value_of("default_branch")
        .ok_or_else(|| "BUG: default_branch has no default".to_owned())?;

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
