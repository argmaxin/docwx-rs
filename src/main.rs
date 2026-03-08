/*  docWX: A minimal, performant book-keeping, authoring and documentation tool.
Copyright (C) 2026  argmaxin

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.*/

use crate::source::Document;
use crate::source::Parser;
use clap::ArgAction;
use std::io::Write;
use std::process::ExitCode;

mod source;
mod table;

const MAX_VALUE: &str = const_str::format!("{}", usize::MAX);

use ignore::WalkBuilder;
use std::io::{self, BufRead};
use std::path::PathBuf;

fn read_stdin_paths() -> Vec<PathBuf> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    reader
        .lines()
        .filter_map(Result::ok)
        .map(PathBuf::from)
        .collect()
}

fn parse_paths(paths: Vec<std::path::PathBuf>) -> Vec<Document> {
    paths
        .into_iter()
        .map(|path| Parser::new(&path).and_then(|mut p| p.parse()))
        .collect::<Result<Vec<_>, _>>()
        .expect("parsing failed")
}

fn expand(vals: Vec<&String>, max_depth: usize, git: bool) -> Vec<Document> {
    let mut builder = WalkBuilder::new(vals[0]);

    for dir in &vals[1..] {
        builder.add(dir);
    }

    let paths: Vec<_> = builder
        .max_depth(Some(max_depth))
        .git_ignore(git)
        .git_global(git)
        .git_exclude(git)
        .require_git(true)
        .follow_links(false)
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.into_path())
        .collect();

    paths
        .into_iter()
        .map(|path| Parser::new(&path).and_then(|mut p| p.parse()))
        .collect::<Result<Vec<_>, _>>()
        .expect("parsing failed")
}

fn main() -> ExitCode {
    let matches = clap::Command::new("docwx")
        .name("docwx")
        .author("argmaxin <contact@argmax.in>")
        .version("0.1.3")
        .about("A minimal, performant book-keeping, authoring and documentation tool.")
        .arg(
            clap::Arg::new("depth")
                .long("depth")
                .short('d')
                .help("Set the maximum recursion depth.")
                .value_parser(clap::value_parser!(usize))
                .action(clap::ArgAction::Set)
                .default_missing_value(MAX_VALUE),
        )
        .arg(
            clap::Arg::new("input")
                .long("input")
                .short('i')
                .help("Set the source(s) directory.")
                .action(ArgAction::Append)
                .required(true),
        )
        .arg(
            clap::Arg::new("output")
                .long("output")
                .short('o')
                .help("Set the output directory.")
                .action(ArgAction::Set)
                .required(true),
        )
        .arg(
            clap::Arg::new("list")
                .short('L')
                .help("Read file paths from stdin (use '-' as input)")
                .action(ArgAction::SetTrue)
                .conflicts_with("input")
                .required(true),
        )
        .arg(
            clap::Arg::new("git")
                .long("gitignore")
                .short('g')
                .help("Whether to respect .git(ignore/exclude/info)")
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
        )
        .color(clap::ColorChoice::Auto)
        .get_matches_from(wild::args());

    let depth = matches.get_one("depth");
    let depth: usize = match depth {
        Some(d) => *d,
        _ => usize::MAX,
    };

    let git = matches.get_one("git");
    let git: bool = match git {
        Some(g) => *g,
        _ => false,
    };

    let use_stdin = matches.get_flag("list");

    let docs: Vec<Document> = if use_stdin {
        let paths = read_stdin_paths();
        parse_paths(paths)
    } else {
        let vals: Vec<&String> = matches
            .get_many("input")
            .expect("`input`is required")
            .collect();

        expand(vals, depth, git)
    };
    let d = crate::source::merge_documents(docs);

    if let Some(doc) = d {
        let res = move || -> anyhow::Result<()> {
            let created_dirs = dashmap::DashSet::new();
            let out_dir = std::path::Path::new(matches.get_one::<String>("output").unwrap());

            doc.pages
                .into_iter()
                .try_for_each(|page| -> anyhow::Result<()> {
                    let full_path = out_dir.join(&page.path);

                    if let Some(parent) = full_path.parent()
                        && !created_dirs.contains(parent)
                    {
                        std::fs::create_dir_all(parent)?;
                        created_dirs.insert(parent.to_path_buf());
                    }

                    let file = std::fs::File::create(&full_path)?;
                    let mut writer = std::io::BufWriter::with_capacity(64 * 1024, file);

                    for sec in &page.sections {
                        writer.write_all(sec.buf.as_bytes())?;
                    }
                    writer.flush()?;
                    Ok(())
                })
        };

        match res() {
            Ok(_) => ExitCode::SUCCESS,
            Err(_) => {
                eprintln!("docwx failed....");
                ExitCode::FAILURE
            }
        }
    } else {
        ExitCode::FAILURE
    }
}
