use super::model;
use super::view;
use ansi_term::Color;

pub fn colors(input: &str) -> model::R<view::Colors> {
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

pub fn ss(input: &str) -> model::R<view::StatusSymbols> {
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

pub fn bs(input: &str) -> model::R<view::BranchSymbols> {
    let parts: Vec<&str> = input.split('|').collect();

    match parts.len() {
        2 => Ok(view::BranchSymbols {
            ahead: parts[0],
            behind: parts[1],
        }),
        _ => Err(format!("Unknown input format: {}", input)),
    }
}
