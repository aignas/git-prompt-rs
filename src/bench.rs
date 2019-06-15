#[macro_use]
extern crate criterion;
use criterion::Criterion;
use ansi_term::Color;

mod model;
mod view;

fn bench_discovery(c: &mut Criterion) {
    c.bench_function("discovery", |b| 
        b.iter(|| git2::Repository::discover(".")));
}

fn bench_view(c: &mut Criterion) {
    c.bench_function("view", |b| b.iter(|| {
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
    }));
}

fn bench_branch_status(c: &mut Criterion) {
    let r = git2::Repository::discover(".");
    c.bench_function(
        "branch_status",
        move |b| b.iter(|| r.as_ref()
            .or_else(|e| Err(format!("{:?}", e)))
            .and_then(|r| model::branch_status(r, "master", "master"))));
}

fn bench_repo_status(c: &mut Criterion) {
    let r = git2::Repository::discover(".");
    c.bench_function(
        "repo_status",
        move |b| b.iter(|| r.as_ref()
            .or_else(|e| Err(format!("{:?}", e)))
            .and_then(|r| model::repo_status(r))));
}

fn bench_local_status(c: &mut Criterion) {
    let r = git2::Repository::discover(".");
    c.bench_function(
        "local_status",
        move |b| b.iter(|| r.as_ref()
            .or_else(|e| Err(format!("{:?}", e)))
            .map(|r| model::local_status(r))));
}

criterion_group!(
    benches,
    bench_discovery, bench_view,
    bench_branch_status, bench_repo_status, bench_local_status);
criterion_main!(benches);
