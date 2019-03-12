/*
 * Copyright (C) 2019 Jan Felix Langenbach
 *
 * This file is part of bfrun.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http: //www.gnu.org/licenses/>.
 */

use crate::{
    ops::OpList,
    types::{Stack, Strip},
};
use std::io::{Read, Write};

pub struct Context<'a> {
    pub strip: Strip,
    pub loops: Stack<OpList>,
    pub mode: Mode,
    pub bfin: Box<Read + 'a>,
    pub bfout: Box<Write + 'a>,
}

pub enum Mode {
    File,
    Stream,
}

impl<'a> Context<'a> {
    pub fn new<R, W>(bfin: R, bfout: W, mode: Mode) -> Self
    where
        R: Read + 'a,
        W: Write + 'a,
    {
        let strip = Strip::new();
        let loops = Stack::new();
        let bfin = Box::from(bfin);
        let bfout = Box::from(bfout);
        Self {
            strip,
            loops,
            mode,
            bfin,
            bfout,
        }
    }
}
