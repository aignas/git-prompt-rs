extern crate clap;
use clap::Clap;
mod app;
mod examples;
mod model;
mod parse;
mod view;

pub fn run() -> model::R<()> {
    let opts = app::Opts::parse();

    // convert from the apps params into model;
    let cs = parse::colors(&opts.colorscheme)?;
    let bs = parse::bs(&opts.branch_symbols)?;
    let ss = parse::ss(&opts.status_symbols)?;

    if opts.examples {
        print!("{}", examples::all().with_style(&cs, &bs, &ss));
        return Ok(());
    }

    let repo = git2::Repository::discover(&opts.path).or_else(|e| Err(format!("{:?}", e)))?;
    let r = model::repo_status(&repo)?;
    let prompt = view::Prompt::new(&r).with_style(&cs, &bs, &ss);

    if opts.print_updates {
        let current = format!("{}", prompt);
        println!("{}", current);
        let prompt = prompt.with_branch(
            r.branch
                .as_ref()
                .and_then(|b| model::branch_status(&repo, b, &opts.default_branch).ok()),
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
                    r.branch.as_ref().and_then(|b| model::branch_status(
                        &repo,
                        b,
                        &opts.default_branch
                    )
                    .ok()),
                )
                .with_local(Some(model::local_status(&repo)))
        );
    }
    Ok(())
}
