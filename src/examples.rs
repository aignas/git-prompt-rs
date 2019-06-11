use super::model;
use super::view;
use std::fmt::{self, Display, Formatter};
use std::string::String;

pub fn all<'a>() -> Examples<'a> {
    let master = Some("master".to_owned());
    let clean = git2::RepositoryState::Clean;
    let rebase = git2::RepositoryState::Rebase;

    fn b(ahead: usize, behind: usize) -> Option<model::BranchStatus> {
        Some(model::BranchStatus { ahead, behind })
    }
    fn s(staged: usize, unstaged: usize, unmerged: usize, untracked: usize) -> model::LocalStatus {
        model::LocalStatus {
            staged,
            unstaged,
            unmerged,
            untracked,
        }
    }

    Examples::new()
        .add("after 'git init'", None, clean, None, s(0, 3, 0, 0))
        .add("ok", master.clone(), clean, b(0, 0), s(0, 0, 0, 0))
        .add("stage", master.clone(), clean, b(0, 0), s(3, 0, 0, 0))
        .add("partial", master.clone(), clean, b(0, 0), s(3, 12, 0, 0))
        .add(
            "conflicts",
            Some("a83e2a3f".to_owned()),
            rebase,
            b(0, 3),
            s(0, 2, 1, 0),
        )
        .add("rebase", master.clone(), rebase, b(0, 3), s(0, 3, 0, 0))
        .add("diverged", master, rebase, b(12, 3), s(0, 0, 0, 3))
}

pub struct Examples<'a> {
    examples: std::collections::HashMap<String, view::Prompt<'a>>,
}

impl<'a> Examples<'a> {
    pub fn new() -> Examples<'a> {
        use std::collections::HashMap;
        Examples {
            examples: HashMap::new(),
        }
    }

    fn add(
        mut self,
        key: &str,
        br: Option<String>,
        state: git2::RepositoryState,
        branch: Option<model::BranchStatus>,
        local: model::LocalStatus,
    ) -> Examples<'a> {
        self.examples.insert(
            key.to_string(),
            view::Prompt::new(&model::RepoStatus { branch: br, state })
                .with_branch(branch)
                .with_local(Some(local)),
        );
        self
    }

    pub fn with_style(
        mut self,
        c: &'a view::Colors,
        bs: &'a view::BranchSymbols,
        ss: &'a view::StatusSymbols,
    ) -> Examples<'a> {
        self.examples = self
            .examples
            .iter()
            .map(|(l, p)| (l.to_owned(), p.with_style(c, bs, ss)))
            .collect();
        self
    }
}

impl<'a> Display for Examples<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let max_length = self
            .examples
            .keys()
            .map(String::len)
            .max()
            .expect("failed to get the maximum example key length");
        self.examples.iter().for_each(|(l, p)| {
            writeln!(f, "{0:>1$}: {2}", l, max_length, p).unwrap();
        });
        Ok(())
    }
}
