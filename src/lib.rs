extern crate xml;

mod data;
mod dot;
mod error;
mod lola;
mod pnml;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;

use data::{Arc, Place, Transition};
use error::PetriError;

pub type Result<T> = std::result::Result<T, PetriError>;
pub use crate::dot::*;
pub use crate::lola::*;
pub use crate::pnml::*;

#[derive(Debug, Clone)]
pub struct PetriNet {
    places: Vec<Place>,
    transitions: Vec<Transition>,
    arcs: Vec<Arc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaceRef {
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransitionRef {
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArcRef {
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            preset: HashMap::new(),
            postset: HashMap::new(),
        });
        NodeRef::Place(PlaceRef {
            index: self.places.len() - 1,
        })
    }
    pub fn add_transition(&mut self) -> NodeRef {
        self.transitions.push(Transition {
            name: None,
            preset: HashMap::new(),
            postset: HashMap::new(),
        });
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
        source.add_to_postset(self, sink)?;
        sink.add_to_preset(self, source)?;
        Ok(ArcRef {
            index: self.arcs.len() - 1,
        })
    }

    /// partition the arcs in transition -> place and place -> transition arcs with
    /// the corresponding multiplicity
    pub(crate) fn arcs_partitioned(
        &self,
    ) -> (
        Vec<(NodeRef, NodeRef, usize)>,
        Vec<(NodeRef, NodeRef, usize)>,
    ) {
        self.arcs
            .iter()
            .map(|arc| (arc.source, arc.sink, arc.mult))
            .partition(|(source, _, _)| TransitionRef::try_from(*source).is_ok())
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

    pub fn add_to_preset(self, net: &mut PetriNet, node: NodeRef) -> Result<()> {
        match self {
            NodeRef::Place(place) => {
                let place = net
                    .places
                    .get_mut(place.index)
                    .ok_or(PetriError::PlaceNotFound)?;
                let transition_index = TransitionRef::try_from(node)?;
                if let Some(mult) = place.preset.insert(transition_index, 1) {
                    place.preset.insert(transition_index, mult + 1);
                };
            }
            NodeRef::Transition(transition) => {
                let transition = net
                    .transitions
                    .get_mut(transition.index)
                    .ok_or(PetriError::TransitionNotFound)?;
                let place_index = PlaceRef::try_from(node)?;
                if let Some(mult) = transition.preset.insert(place_index, 1) {
                    transition.preset.insert(place_index, mult + 1);
                };
            }
        }
        Ok(())
    }

    pub fn add_to_postset(self, net: &mut PetriNet, node: NodeRef) -> Result<()> {
        match self {
            NodeRef::Place(place) => {
                let place = net
                    .places
                    .get_mut(place.index)
                    .ok_or(PetriError::PlaceNotFound)?;
                let transition_index = TransitionRef::try_from(node)?;
                if let Some(mult) = place.postset.insert(transition_index, 1) {
                    place.postset.insert(transition_index, mult + 1);
                };
            }
            NodeRef::Transition(transition) => {
                let transition = net
                    .transitions
                    .get_mut(transition.index)
                    .ok_or(PetriError::TransitionNotFound)?;
                let place_index = PlaceRef::try_from(node)?;
                if let Some(mult) = transition.postset.insert(place_index, 1) {
                    transition.postset.insert(place_index, mult + 1);
                };
            }
        }
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

    pub fn preset<'net>(&self, net: &'net PetriNet) -> Result<&'net HashMap<TransitionRef, usize>> {
        Ok(&net
            .places
            .get(self.index)
            .ok_or(PetriError::PlaceNotFound)?
            .preset)
    }

    pub fn postset<'net>(
        &self,
        net: &'net PetriNet,
    ) -> Result<&'net HashMap<TransitionRef, usize>> {
        Ok(&net
            .places
            .get(self.index)
            .ok_or(PetriError::PlaceNotFound)?
            .postset)
    }
}

impl TransitionRef {
    pub fn preset<'net>(&self, net: &'net PetriNet) -> Result<&'net HashMap<PlaceRef, usize>> {
        Ok(&net
            .transitions
            .get(self.index)
            .ok_or(PetriError::TransitionNotFound)?
            .preset)
    }

    pub fn postset<'net>(&self, net: &'net PetriNet) -> Result<&'net HashMap<PlaceRef, usize>> {
        Ok(&net
            .transitions
            .get(self.index)
            .ok_or(PetriError::TransitionNotFound)?
            .postset)
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
