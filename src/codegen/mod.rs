//! This module contains the different Haumea code generators.

pub mod c;

pub trait CodeGen {
    fn compile(&mut self) -> String;
}