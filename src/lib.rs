extern crate xml;

mod data;
mod error;
mod lola;
mod pnml;

use std::convert::TryFrom;

use data::{Arc, Place, Transition};
use error::PetriError;

pub type Result<T> = std::result::Result<T, PetriError>;

#[derive(Debug, Clone)]
pub struct PetriNet {
    places: Vec<Place>,
    transitions: Vec<Transition>,
    arcs: Vec<Arc>,
}

#[derive(Debug, Clone, Copy)]
pub struct PlaceRef {
    index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct TransitionRef {
    index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ArcRef {
    index: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeRef {
    Place(PlaceRef),
    Transition(TransitionRef),
}

impl PetriNet {
    pub fn new() -> Self {
        PetriNet {
            places: Vec::new(),
            transitions: Vec::new(),
            arcs: Vec::new(),
        }
    }

    pub fn add_place(&mut self) -> NodeRef {
        self.places.push(Place {
            name: None,
            marking: 0,
        });
        NodeRef::Place(PlaceRef {
            index: self.places.len() - 1,
        })
    }
    pub fn add_transition(&mut self) -> NodeRef {
        self.transitions.push(Transition { name: None });
        NodeRef::Transition(TransitionRef {
            index: self.transitions.len() - 1,
        })
    }

    pub fn add_arc(&mut self, source: NodeRef, sink: NodeRef) -> Result<ArcRef> {
        check_bipartition(source, sink)?;
        self.arcs.push(Arc {
            name: None,
            source,
            sink,
            mult: 1,
        });
        Ok(ArcRef {
            index: self.arcs.len() - 1,
        })
    }
}

impl NodeRef {
    pub fn name(self, net: &mut PetriNet, name: String) -> Result<()> {
        let place = PlaceRef::try_from(self);
        let trans = TransitionRef::try_from(self);
        let node_name = if place.is_ok() {
            &mut net
                .places
                .get_mut(place?.index)
                .ok_or(PetriError::PlaceNotFound)?
                .name
        } else {
            &mut net
                .transitions
                .get_mut(trans?.index)
                .ok_or(PetriError::TransitionNotFound)?
                .name
        };
        *node_name = Some(name);
        Ok(())
    }
}

impl PlaceRef {
    pub fn marking(self, net: &mut PetriNet, marking: usize) -> Result<()> {
        net.places
            .get_mut(self.index)
            .ok_or(PetriError::PlaceNotFound)?
            .marking = marking;
        Ok(())
    }
}

impl ArcRef {
    pub fn name(self, net: &mut PetriNet, name: String) -> Result<()> {
        net.arcs
            .get_mut(self.index)
            .ok_or(PetriError::ArcNotFound)?
            .name = Some(name);
        Ok(())
    }
    pub fn multiplicity(self, net: &mut PetriNet, mult: usize) -> Result<()> {
        net.arcs
            .get_mut(self.index)
            .ok_or(PetriError::ArcNotFound)?
            .mult = mult;
        Ok(())
    }
}

impl TryFrom<NodeRef> for TransitionRef {
    type Error = PetriError;

    fn try_from(value: NodeRef) -> Result<Self> {
        match value {
            NodeRef::Place(_) => Err(PetriError::InvalidData(
                "conversion from place node to transition reference".into(),
            )),
            NodeRef::Transition(t) => Ok(t),
        }
    }
}

impl TryFrom<NodeRef> for PlaceRef {
    type Error = PetriError;

    fn try_from(value: NodeRef) -> Result<Self> {
        match value {
            NodeRef::Transition(_) => Err(PetriError::InvalidData(
                "conversion from transition node to place reference".into(),
            )),
            NodeRef::Place(p) => Ok(p),
        }
    }
}

fn check_bipartition(a: NodeRef, b: NodeRef) -> Result<()> {
    match a {
        NodeRef::Place(_) => match b {
            NodeRef::Place(_) => Err(PetriError::BipartitionViolation),
            NodeRef::Transition(_) => Ok(()),
        },
        NodeRef::Transition(_) => match b {
            NodeRef::Place(_) => Ok(()),
            NodeRef::Transition(_) => Err(PetriError::BipartitionViolation),
        },
    }
}
