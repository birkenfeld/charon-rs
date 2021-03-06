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

pub mod ast;
pub mod runtime;

pub use charon_parsers::{parse_tc2_project, parse_tc3_project};

#[test]
fn test_tc3() {
    let proj = parse_tc3_project("CCMHTS/CCMHTS.plcproj");
    println!("{}", proj.0.pous.len());
    println!("{:#?}", proj.1);
}

#[test]
fn test_tc2() {
    let proj = parse_tc2_project("exps");
    println!("{}", proj.0.pous.len());
    println!("{:#?}", proj.1);
}
