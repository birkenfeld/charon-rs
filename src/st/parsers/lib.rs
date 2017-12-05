// *****************************************************************************
// Charon: Beckhoff TwinCat/ST testing and simulation tools
// Copyright (c) 2017 by the contributors (see AUTHORS)
//
// This program is free software; you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation; either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc.,
// 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//
// Module authors:
//   Georg Brandl <g.brandl@fz-juelich.de>
//
// *****************************************************************************

#![feature(box_syntax)]

extern crate regex;
extern crate walkdir;
extern crate encoding;
extern crate elementtree as etree;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

pub mod ast;
mod tc2;
mod tc3;

use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use regex::{Regex, Captures};
use encoding::all::WINDOWS_1252;
use failure::Error;

// helper for the parser
fn app<T>(mut v: Vec<T>, x: T) -> Vec<T> { v.push(x); v }

lazy_static! {
    static ref COMMENT_RX: Regex = Regex::new(r"(?s)\(\*.*?\*\)").unwrap();
    static ref DIRECTIVE_RX: Regex = Regex::new(r"(?s)\{.*?\}").unwrap();
}

/// Parse an ST source string.
fn prepare_input<'a>(input: &'a str) -> String {
    let spaces = |cap: &Captures| " ".repeat(cap[0].len());
    let input = COMMENT_RX.replace_all(&input, &spaces);
    DIRECTIVE_RX.replace_all(&input, spaces).into_owned()
}

/// Parse a single TwinCat 2 `.exp` file.
pub fn parse_tc2_file<P: AsRef<Path>>(path: P) -> Result<ast::POU, Error> {
    let mut v = Vec::new();
    fs::File::open(path.as_ref())?.read_to_end(&mut v)?;
    let input = encoding::decode(&v, encoding::DecoderTrap::Strict, WINDOWS_1252)
        .0.map_err(|_| format_err!("Could not decode source file"))?;
    tc2::parse_file(&prepare_input(&input)).map_err(|e| format_err!("Parse error: {}", e))
}

/// Parse a whole TwinCat 2 export directory.
pub fn parse_tc2_project<P: AsRef<Path>>(path: P) -> (ast::Project, Vec<(PathBuf, Error)>) {
    let mut project = ast::Project { pous: vec![] };
    let mut errors = vec![];
    for entry in walkdir::WalkDir::new(path) {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if ext == "exp" || ext == "EXP" {
                    match parse_tc2_file(entry.path()) {
                        Ok(pou) => project.pous.push(pou),
                        Err(err) => errors.push((entry.path().to_path_buf(), err)),
                    }
                }
            }
        }
    }
    (project, errors)
}

fn read_etree<P: AsRef<Path>>(path: P, top: &str) -> Result<etree::Element, Error> {
    let mut file = fs::File::open(path.as_ref())?;
    let mut bom = [0; 3];
    file.read_exact(&mut bom)?;
    if bom != [0xef, 0xbb, 0xbf] {
        file.seek(SeekFrom::Start(0))?;
    }
    let tree = etree::Element::from_reader(file)?;
    if tree.tag().name() == top {
        Ok(tree)
    } else {
        bail!("Not a {} file", top)
    }
}

/// Parse a single TwinCat 3 `.TcXXX` file.
pub fn parse_tc3_file<P: AsRef<Path>>(path: P) -> Result<Option<ast::POU>, Error> {
    let tree = read_etree(&path, "TcPlcObject")?;
    let mut input = String::new();
    let pou = tree.get_child(0).unwrap();
    let mut name_override = None;
    match pou.tag().name() {
        "POU" => {
            let decl = pou.find("Declaration").ok_or_else(
                || format_err!("No declaration tag found in {}", path.as_ref().display()))?;
            let impl_ = pou.navigate(&["Implementation", "ST"]).ok_or_else(
                || format_err!("No implementation tag found in {}", path.as_ref().display()))?;
            input.push_str(decl.text());
            input.push('\n');
            input.push_str(impl_.text());
        },
        "DUT" => {
            let decl = pou.find("Declaration").ok_or_else(
                || format_err!("No declaration tag found in {}", path.as_ref().display()))?;
            input.push_str(decl.text());
        },
        "GVL" => {
            let decl = pou.find("Declaration").ok_or_else(
                || format_err!("No declaration tag found in {}", path.as_ref().display()))?;
            input.push_str(decl.text());
            name_override = pou.get_attr("Name");
        },
        "GlobalImagePool" | "GlobalTextList" | "Visu" | "VisuManager" | "Task" => {
            return Ok(None);
        },
        typ => {
            bail!("Not a recognized POU: {}", typ);
        },
    }
    let mut pou = tc3::parse_file(&prepare_input(&input))
        .map_err(|e| format_err!("Parse error: {}", e))?;
    if let Some(name) = name_override {
        pou.0 = name.into();
    }
    Ok(Some(pou))
}

/// Parse a TwinCat 3 `.plcproj` project.
pub fn parse_tc3_project<P: AsRef<Path>>(path: P) -> (ast::Project, Vec<(PathBuf, Error)>) {
    let mut project = ast::Project { pous: vec![] };
    let mut errors = vec![];
    let basedir = path.as_ref().parent().unwrap_or(Path::new("."));
    let tree = match read_etree(&path, "Project") {
        Ok(tree) => tree,
        Err(e) => {
            errors.push((path.as_ref().to_path_buf(), e));
            return (project, errors);
        }
    };
    let ns = "http://schemas.microsoft.com/developer/msbuild/2003";
    for group in tree.find_all((ns, "ItemGroup")) {
        for comp in group.find_all((ns, "Compile")) {
            if let Some(relpath) = comp.get_attr("Include") {
                let fullpath = basedir.join(relpath.replace("\\", "/"));
                println!("{}", fullpath.display());
                match parse_tc3_file(&fullpath) {
                    Ok(Some(pou)) => project.pous.push(pou),
                    Ok(None) => continue,
                    Err(err) => errors.push((fullpath, err)),
                }
            }
        }
    }
    (project, errors)
}
