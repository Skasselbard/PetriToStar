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
                let line = if let Some(name) = &self.places.get(p).unwrap().name {
                    format!(
                        "    {}{} [shape=\"circle\" label=\"{}\"];\n",
                        PLACE_PREFIX, p, name
                    )
                } else {
                    format!("    {}{} [shape=\"circle\"];\n", PLACE_PREFIX, p)
                };
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
                let line = if let Some(name) = &self.transitions.get(t).unwrap().name {
                    format!(
                        "    {}{} [shape=\"box\" label=\"{}\"];\n",
                        TRANSITION_PREFIX, t, name
                    )
                } else {
                    format!("    {}{} [shape=\"box\"];\n", TRANSITION_PREFIX, t)
                };
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
