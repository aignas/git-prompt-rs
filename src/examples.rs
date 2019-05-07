use super::model;
use super::view;
use std::fmt::{self, Display, Formatter};

pub fn all<'a>() -> Examples<'a> {
    let master = Some("master");
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
}

pub struct Examples<'a> {
    examples: std::collections::HashMap<String, view::Prompt<'a>>,
    c: Option<&'a view::Colors>,
    bs: Option<&'a view::BranchSymbols<'a>>,
    ss: Option<&'a view::StatusSymbols<'a>>,
}

impl<'a> Examples<'a> {
    pub fn new() -> Examples<'a> {
        use std::collections::HashMap;
        Examples {
            examples: HashMap::new(),
            c: None,
            bs: None,
            ss: None,
        }
    }

    fn add(
        mut self,
        key: &str,
        br: Option<&str>,
        state: git2::RepositoryState,
        branch: Option<model::BranchStatus>,
        local: model::LocalStatus,
    ) -> Examples<'a> {
        let repo = model::RepoStatus {
            branch: br.map(|s| s.to_owned()),
            state,
        };
        self.examples.insert(
            key.to_string(),
            view::Prompt::new(&repo)
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
        self.c = Some(c);
        self.bs = Some(bs);
        self.ss = Some(ss);
        self
    }
}

impl<'a> Display for Examples<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let max_length = self
            .examples
            .keys()
            .map(|label| label.len())
            .max()
            .expect("failed to get the maximum example key length");
        for (label, p) in &self.examples {
            writeln!(
                f,
                "{0:>1$}: {2}",
                label,
                max_length,
                view::print(
                    p.clone(),
                    &self.c.unwrap(),
                    &self.bs.unwrap(),
                    &self.ss.unwrap(),
                )
            )?;
        }
        Ok(())
    }
}
