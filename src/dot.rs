use crate::{PetriNet, PlaceRef, TransitionRef};
use std::convert::TryFrom;

const PLACE_PREFIX: &str = "p_";
const TRANSITION_PREFIX: &str = "t_";

impl PetriNet {
    pub fn to_dot_string(&self) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();
        self.write_dot(&mut writer)?;
        Ok(String::from_utf8(writer).expect("Document generated non UTF-8 string"))
    }

    pub fn to_dot<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        self.write_dot(writer)?;
        Ok(())
    }

    fn write_dot<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        writer.write("digraph petrinet {\n".as_bytes())?;
        self.write_dot_places(writer)?;
        self.write_dot_transitions(writer)?;
        self.write_dot_arcs(writer)?;
        writer.write("}".as_bytes())?;
        Ok(())
    }

    fn write_dot_places<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        if !self.places.is_empty() {
            for p in 0..self.places.len() {
                let marking = self.places.get(p).unwrap().marking;
                let marking = if marking > 0 {
                    let mut ret;
                    if marking < 5 {
                        ret = String::new();
                        for _ in 0..marking {
                            ret.push_str("•");
                        }
                    } else {
                        ret = marking.to_string();
                    };
                    Some(ret)
                } else {
                    None
                };
                let line = format_dot_node(
                    PLACE_PREFIX,
                    p,
                    "circle",
                    &marking,
                    &self.places.get(p).unwrap().name,
                );
                writer.write(line.as_bytes())?;
            }
        }
        Ok(())
    }

    fn write_dot_transitions<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        println!("empty t?:{}", self.transitions.is_empty());
        println!("{}", self.transitions.len());
        if !self.transitions.is_empty() {
            for t in 0..self.transitions.len() {
                let line = format_dot_node(
                    TRANSITION_PREFIX,
                    t,
                    "box",
                    &self.transitions.get(t).unwrap().name,
                    &None,
                );
                writer.write(line.as_bytes())?;
            }
        }
        Ok(())
    }

    fn write_dot_arcs<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        let (tp, pt) = self.arcs_partitioned();
        for (t, p, mult) in tp {
            if mult > 0 {
                let t = TransitionRef::try_from(t).unwrap();
                let p = PlaceRef::try_from(p).unwrap();
                let line = if mult > 1 {
                    format!(
                        "    {}{} -> {}{} [label=\"{}\"];\n",
                        TRANSITION_PREFIX, t.index, PLACE_PREFIX, p.index, mult
                    )
                } else {
                    format!(
                        "    {}{} -> {}{};\n",
                        TRANSITION_PREFIX, t.index, PLACE_PREFIX, p.index
                    )
                };
                writer.write(line.as_bytes())?;
            }
        }
        for (p, t, mult) in pt {
            if mult > 0 {
                let t = TransitionRef::try_from(t).unwrap();
                let p = PlaceRef::try_from(p).unwrap();
                let line = if mult > 1 {
                    format!(
                        "    {}{} -> {}{} [label=\"{}\"];\n",
                        PLACE_PREFIX, p.index, TRANSITION_PREFIX, t.index, mult
                    )
                } else {
                    format!(
                        "    {}{} -> {}{};\n",
                        PLACE_PREFIX, p.index, TRANSITION_PREFIX, t.index
                    )
                };
                writer.write(line.as_bytes())?;
            }
        }
        Ok(())
    }
}
fn format_dot_node(
    prefix: &str,
    index: usize,
    shape: &str,
    label: &Option<String>,
    caption: &Option<String>,
) -> String {
    let label = if let Some(label) = label {
        format!("label=\"{}\" ", label)
    } else {
        String::new()
    };
    let caption = if let Some(caption) = caption {
        format!("xlabel=\"{}\" ", caption)
    } else {
        String::new()
    };
    format!(
        "    {}{} [shape=\"{}\" {} {}];\n",
        prefix, index, shape, label, caption
    )
}
