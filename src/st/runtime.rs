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

use std::collections::HashMap;


/// Represents a whole PLC runtime.
pub struct Runtime {
    tasks: Vec<Task>,
}

pub type Var = usize;
pub type Func = usize;

/// Represents a single PLC task.
pub struct Task {
    var_names: HashMap<Var, String>,
    func_names: HashMap<Func, String>,
    vars: Vec<Alloc>,
    functions: Vec<Program>,

    stack: Vec<Data>,
    memory: Box<[u8]>,
}

/// Runtime representation of a PLC program.
pub struct Program {
    code: Vec<Instr>,
}

/// A variable allocation.
pub struct Alloc {
    offset: usize,
    size: usize,
}

/// A piece of data.
pub enum Data {
    Bool(bool),
    UInt(u32),
    SInt(i32),
    Float(f32),
}

/// An instruction.
pub enum Instr {
    Store(Var),
    Load(Var),
    BinOp(fn(Data, Data) -> Data),
    UnOp(fn(Data) -> Data),
    Call(Func),
}

impl Task {
    pub fn run_cycle(&mut self) {
        for instr in &self.functions[0] {
            match *instr {
                Instr::Load(v) => {
                    self.stack.push(self.vars[v].load(&self.memory));
                }
                Instr::Store(v) => {
                    let v = self.stack.pop().unwrap();
                    self.vars[v].store(&mut self.memory, v);
                }
                Instr::BinOp(func) => {
                    let v = self.stack.pop().unwrap();
                    let w = self.stack.pop().unwrap();
                    self.stack.push(func(v, w));
                }
                Instr::UnOp(func) => {
                    let v = self.stack.pop().unwrap();
                    self.stack.push(func(v));
                }
                Instr::Call(fb) => {
                    
                }
            }
        }
        
    }
}
