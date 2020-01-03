use crate::{PetriNet, TransitionRef};

impl PetriNet {
    pub fn to_lola_string(&self) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();
        self.write_lola(&mut writer)?;
        Ok(String::from_utf8(writer).expect("Document generated non UTF-8 string"))
    }

    pub fn to_lola<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        if self.print_unconnected_nodes().is_err() {
            panic!("Cannot find all nodes: this is a bug!");
        };
        self.write_lola(writer)?;
        Ok(())
    }

    fn write_lola<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        self.write_lola_places(writer)?;
        self.write_lola_markings(writer)?;
        self.write_lola_transitions(writer)
    }

    /// ```
    /// PLACE
    ///     p_1,
    ///     ..
    ///     p_n;
    /// ```
    fn write_lola_places<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        const PREFIX: &str = "p_";
        if !self.places.is_empty() {
            writer.write("PLACE\n".as_bytes())?;
            if self.places.len() > 2 {
                for place in 0..self.places.len() - 1 {
                    let line = format!("    {},\n", make_id(PREFIX, place));
                    writer.write(line.as_bytes())?;
                }
            };
            // last line has a semicolon
            let place = self.places.len() - 1;
            let line = format!("    {};\n\n", make_id(PREFIX, place));
            writer.write(line.as_bytes())?;
        }
        Ok(())
    }

    /// ```
    /// MARKING
    ///   p_5 : 4,
    ///   p_25 : 1;
    /// ```
    fn write_lola_markings<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        const PREFIX: &str = "p_";
        if !self.places.is_empty() {
            let mut first = true;
            writer.write("MARKING\n".as_bytes())?;
            for place in 0..self.places.len() {
                let marking = self.places.get(place).unwrap().marking;
                let line = if marking > 0 {
                    // first line has no ',' at the front
                    if !first {
                        format!(",\n    {} : {}", make_id(PREFIX, place), marking)
                    } else {
                        first = false;
                        format!("    {} : {}", make_id(PREFIX, place), marking)
                    }
                } else {
                    String::new()
                };
                writer.write(line.as_bytes())?;
            }
            // last line has a semicolon
            writer.write(";\n\n".as_bytes())?;
        }
        Ok(())
    }

    /// ```
    /// TRANSITION
    ///   CONSUME
    ///     p_0 : 1,
    ///     p_1 : 2;
    ///   PRODUCE
    ///     P_15 : 182781;
    ///  
    /// TRANSITION
    /// ...
    /// ```
    fn write_lola_transitions<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        const PREFIX: &str = "t_";
        const PLACE_PREFIX: &str = "p_";
        if !self.transitions.is_empty() {
            for t in 0..self.transitions.len() {
                let line = format!("TRANSITION {}\n", make_id(PREFIX, t));
                writer.write(line.as_bytes())?;
                let mut consume = TransitionRef { index: t }
                    .preset(self)
                    .expect("error in preset")
                    .iter();
                if let Some(first) = consume.next() {
                    writer.write("  CONSUME\n".as_bytes())?;
                    writer.write(
                        format!("    {}{} : {}", PLACE_PREFIX, first.0.index, first.1,).as_bytes(),
                    )?;
                    for (place, mult) in consume {
                        writer.write(
                            format!(",\n    {}{} : {}", PLACE_PREFIX, place.index, mult,)
                                .as_bytes(),
                        )?;
                    }
                    writer.write(";\n".as_bytes())?;
                }
                let mut produce = TransitionRef { index: t }
                    .postset(self)
                    .expect("error in postsst")
                    .iter();
                if let Some(first) = produce.next() {
                    writer.write("  PRODUCE\n".as_bytes())?;
                    writer.write(
                        format!("    {}{} : {}", PLACE_PREFIX, first.0.index, first.1,).as_bytes(),
                    )?;
                    for (place, mult) in produce {
                        writer.write(
                            format!(",\n    {}{} : {}", PLACE_PREFIX, place.index, mult,)
                                .as_bytes(),
                        )?;
                    }
                    writer.write(";\n".as_bytes())?;
                }
            }
        }
        Ok(())
    }
}
fn make_id<'a>(prefix: &str, id: usize) -> String {
    let mut ret = String::from(prefix);
    ret.push_str(&id.to_string());
    ret
}
