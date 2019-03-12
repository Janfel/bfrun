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
