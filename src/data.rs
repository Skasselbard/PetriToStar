use crate::{NodeRef, PlaceRef, TransitionRef};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Place {
    pub name: Option<String>,
    pub marking: usize,
    pub preset: HashMap<TransitionRef, usize>,
    pub postset: HashMap<TransitionRef, usize>,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub name: Option<String>,
    pub preset: HashMap<PlaceRef, usize>,
    pub postset: HashMap<PlaceRef, usize>,
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub name: Option<String>,
    pub source: NodeRef,
    pub sink: NodeRef,
    /// multiplicity: amount of tokens that get consumed/produced
    pub mult: usize,
}
