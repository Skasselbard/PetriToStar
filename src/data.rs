use crate::NodeRef;

#[derive(Debug, Clone)]
pub struct Place {
    pub name: Option<String>,
    pub marking: usize,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub name: Option<String>,
    pub source: NodeRef,
    pub sink: NodeRef,
    /// multiplicity: amount of tokens that get consumed/produced
    pub mult: usize,
}
