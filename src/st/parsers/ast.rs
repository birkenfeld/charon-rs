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


#[derive(Debug)]
pub struct Project {
    pub pous: Vec<POU>,
}

#[derive(Debug)]
pub struct POU(pub String, pub POUType);

#[derive(Debug)]
pub enum POUType {
    Globals {
        constant: bool,
        vars: Vec<VarDef>,
    },
    Struct {
        members: Vec<VarDef>,
    },
    Typedef {
        alias: Type,
    },
    Program {
        body: Vec<Stmt>,
        vars: Vec<VarDef>,
    },
    FBlock {
        body: Vec<Stmt>,
        vars: Vec<VarBlock>,
    },
    Function {
        rtype: Type,
        vars: Vec<VarBlock>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug)]
pub struct VarBlock(pub VarType, pub Vec<VarDef>);

#[derive(Debug)]
pub enum VarType {
    In,
    Out,
    InOut,
    Local,
}

#[derive(Debug)]
pub struct VarDef {
    pub name: String,
    pub loc: Option<String>, // TODO
    pub typ: Type,
    pub default: Option<Expr>,
}

#[derive(Debug)]
pub enum Type {
    Simple(String),
    Array(Box<Type>, i64, i64),
    String(usize),
}

#[derive(Debug)]
pub enum Stmt {
    Empty,
    Exit,
    If(Box<Expr>, Vec<Stmt>, Vec<Stmt>),
    Case(Box<Expr>, Vec<Case>, Vec<Stmt>),
    While(Box<Expr>, Vec<Stmt>),
    Assign(Box<Expr>, Box<Expr>),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub struct Case(pub Vec<CaseExpr>, pub Vec<Stmt>);

#[derive(Debug)]
pub enum CaseExpr {
    Single(Expr),
    Range(Expr, Expr),
}

#[derive(Debug)]
pub enum Kwarg {
    In(String, Expr),
    Out(String, Expr),
    None(String),
}

#[derive(Debug)]
pub enum Expr {
    Name(String),
    Lit(Lit),
    List(Vec<Expr>),
    Unary(UnOp, Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Call(String, Vec<Expr>),
    CallFB(String, Vec<Kwarg>),
    Member(Box<Expr>, String),
    Bit(Box<Expr>, u16),
    Sub(Box<Expr>, Box<Expr>),
    Initializer(Vec<(String, Expr)>),
}

#[derive(Debug)]
pub enum UnOp {
    Neg,
}

#[derive(Debug)]
pub enum BinOp {
    Or,
    Xor,
    And,
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum Lit {
    Bool(bool),
    Int(u16, i64),
    Float(f64),
    Str(String),
    Time(String), // TODO
}
