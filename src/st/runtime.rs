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

#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use byteorder::{LE, ByteOrder};


/// Represents a whole PLC runtime.
pub struct Runtime {
    tasks: Vec<Task>,
}

pub type Var = usize;
pub type Func = usize;

/// Represents a single PLC task at runtime.
pub struct Task {
    program: Program,
    stack: Vec<Data>,
    memory: Box<[u8]>,
}

/// Representation of a PLC program (collection of functions).
pub struct Program {
    vars: Vec<VarAlloc>,
    functions: Vec<Function>,
    var_names: HashMap<Var, String>,
    func_names: HashMap<Func, String>,
}

/// Representation of a PLC function (block).
pub struct Function {
    pub code: Vec<Instr>,
}

/// A variable allocation.
pub struct VarAlloc {
    pub offset: usize,
    pub size: usize,
}

/// A piece of data.
pub struct Data(u32);

/// An instruction.
#[derive(Clone, Copy)]
pub enum Instr {
    Store(Var),
    StoreBit(Var, usize),
    Load(Var),
    LoadBit(Var, usize),
    BinOp(fn(Data, Data) -> Data),
    UnOp(fn(Data) -> Data),
    Jump(usize),
    JumpIf(usize),
    Call(Func),
    Return,
}

impl Task {
    pub fn run_cycle(&mut self) {
        self.run_function(0)
    }

    fn run_function(&mut self, idx: usize) {
        let mut pc = 0;
        loop {
            let instr = self.program.functions[idx].code[pc];
            match instr {
                Instr::Load(var) => {
                    self.stack.push(self.program.vars[var].load(&self.memory));
                }
                Instr::LoadBit(var, bit) => {
                    let mut v = self.program.vars[var].load(&self.memory);
                    self.stack.push(v.bit(bit));
                }
                Instr::Store(var) => {
                    let v = self.stack.pop().unwrap();
                    self.program.vars[var].store(&mut self.memory, v);
                }
                Instr::StoreBit(var, bit) => {
                    let b = self.stack.pop().unwrap();
                    let mut v = self.program.vars[var].load(&self.memory);
                    v.set_bit(bit, b);
                    self.program.vars[var].store(&mut self.memory, v);
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
                Instr::Jump(new_pc) => {
                    pc = new_pc;
                    continue;
                }
                Instr::Call(fb) => {
                    self.run_function(fb);
                }
                Instr::Return => {
                    return;
                }
            }
            pc += 1;
        }
    }
}

impl VarAlloc {
    fn load(&self, mem: &[u8]) -> Data {
        Data(match self.size {
            1 => mem[self.offset] as u32,
            2 => LE::read_u16(&mem[self.offset..]) as u32,
            4 => LE::read_u32(&mem[self.offset..]),
            _ => panic!("Variable too large to load")
        })
    }

    fn store(&self, mem: &mut [u8], data: Data) {
        match self.size {
            1 => mem[self.offset] = data.0 as u8,
            2 => LE::write_u16(&mut mem[self.offset..], data.0 as u16),
            4 => LE::write_u32(&mut mem[self.offset..], data.0),
            _ => panic!("Variable too large to store")
        }
    }
}

impl Data {
    fn bit(&self, bit: usize) -> Data {
        Data((self.0 >> bit) & 1)
    }

    fn set_bit(&mut self, bit: usize, val: Data) {
        if val.0 != 0 {
            *self = Data(self.0 | (1 << bit));
        } else {
            *self = Data(self.0 & !(1 << bit));
        }
    }
}
