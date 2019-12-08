use crate::data::{Arc, Place, Transition};
use crate::{PetriNet, Result};
use xml;
use xml::writer::{EmitterConfig, XmlEvent};

const ARC_PREFIX: &str = "arc_";
const PLACE_PREFIX: &str = "place_";
const TRANS_PREFIX: &str = "transition_";

impl PetriNet {
    pub fn to_pnml_string(&self) -> Result<String> {
        let mut writer = Vec::new();
        let mut xml_writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut writer);
        self.write_xml(&mut xml_writer)?;
        Ok(String::from_utf8(writer).expect("Document generated non UTF-8 string"))
    }

    pub fn to_pnml<T>(&self, writer: &mut T) -> Result<()>
    where
        T: std::io::Write,
    {
        let mut xml_writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(writer);
        self.write_xml(&mut xml_writer)?;
        Ok(())
    }

    fn write_xml<T>(&self, writer: &mut xml::writer::EventWriter<T>) -> Result<()>
    where
        T: std::io::Write,
    {
        writer.write(
            XmlEvent::start_element("pnml")
                .default_ns("http://www.pnml.org/version-2009/grammar/pnml"),
        )?;
        writer.write(
            XmlEvent::start_element("net")
                .attr("id", "net0")
                .attr("type", "http://www.pnml.org/version-2009/grammar/ptnet"),
        )?;
        writer.write(XmlEvent::start_element("page").attr("id", "page0"))?;
        if !self.places.is_empty() {
            for place in 0..self.places.len() - 1 {
                self.places.get(place).unwrap().to_xml(writer, place)?;
            }
        };
        if !self.transitions.is_empty() {
            for trans in 0..self.transitions.len() - 1 {
                self.transitions.get(trans).unwrap().to_xml(writer, trans)?;
            }
        };
        if !self.arcs.is_empty() {
            for arc in 0..self.arcs.len() - 1 {
                self.arcs.get(arc).unwrap().to_xml(writer, arc)?;
            }
        };
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

fn name_to_xml<T>(writer: &mut xml::writer::EventWriter<T>, name: &Option<String>) -> Result<()>
where
    T: std::io::Write,
{
    if let Some(name) = &name {
        writer.write(XmlEvent::start_element("name"))?;
        writer.write(XmlEvent::start_element("text"))?;
        writer.write(XmlEvent::Characters(name))?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
    };
    Ok(())
}

impl Arc {
    fn to_xml<T>(&self, writer: &mut xml::writer::EventWriter<T>, index: usize) -> Result<()>
    where
        T: std::io::Write,
    {
        use crate::NodeRef;
        fn get_node_id(node: NodeRef) -> String {
            match node {
                NodeRef::Place(p) => format!("{}{}", PLACE_PREFIX, p.index),
                NodeRef::Transition(t) => format!("{}{}", TRANS_PREFIX, t.index),
            }
        }
        let id = format!("{}{}", ARC_PREFIX, index);
        let source = get_node_id(self.source);
        let target = get_node_id(self.sink);
        let weight = self.mult.to_string();
        let start_element = XmlEvent::start_element("arc")
            .attr("source", &source)
            .attr("target", &target);
        let start_element = start_element.attr("id", &id);
        writer.write(start_element)?;
        {
            name_to_xml(writer, &self.name)?;
            writer.write(XmlEvent::start_element("inscription"))?;
            {
                writer.write(XmlEvent::start_element("text"))?;
                {
                    writer.write(XmlEvent::Characters(&weight))?;
                }
                writer.write(XmlEvent::end_element())?;
            }
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl Place {
    fn to_xml<T>(&self, writer: &mut xml::writer::EventWriter<T>, index: usize) -> Result<()>
    where
        T: std::io::Write,
    {
        let id = format!("{}{}", PLACE_PREFIX, index);
        let marking = self.marking.to_string();
        let start_element = XmlEvent::start_element("place");
        let start_element = start_element.attr("id", &id);
        writer.write(start_element)?;
        {
            name_to_xml(writer, &self.name)?;
            if self.marking > 0 {
                writer.write(XmlEvent::start_element("initialMarking"))?;
                {
                    writer.write(XmlEvent::start_element("text"))?;
                    {
                        writer.write(XmlEvent::Characters(&marking))?;
                    }
                    writer.write(XmlEvent::end_element())?;
                }
                writer.write(XmlEvent::end_element())?;
            }
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl Transition {
    fn to_xml<T>(&self, writer: &mut xml::writer::EventWriter<T>, index: usize) -> Result<()>
    where
        T: std::io::Write,
    {
        let id = format!("{}{}", TRANS_PREFIX, index);
        let start_element = XmlEvent::start_element("transition");
        let start_element = start_element.attr("id", &id);
        writer.write(start_element)?;
        {
            name_to_xml(writer, &self.name)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}
