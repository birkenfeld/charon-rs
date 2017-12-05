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

extern crate byteorder;
extern crate regex;
extern crate lalrpop_util;

pub mod st;

#[test]
fn test_it() {
    use std::io::*;
    use std::fs;
    let rx = regex::Regex::new(r"(?s)\(\*.*?\*\)").unwrap();

    for file in fs::read_dir("exps").unwrap() {
        let file = file.unwrap();
        println!("{}", file.path().display());
        let mut v = Vec::new();
        fs::File::open(file.path()).unwrap().read_to_end(&mut v).unwrap();
        let s = String::from_utf8_lossy(&v);
        let s2 = rx.replace_all(&s, |cap: &regex::Captures| " ".repeat(cap[0].len()));
        if let Err(e) = st::parse_file(&s2) {
            println!("{}", e);
        }
        println!();
    }
    panic!("...");
}
