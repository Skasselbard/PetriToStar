use std::error::Error;

pub enum PetriError {
    BipartitionViolation,
    PlaceNotFound,
    TransitionNotFound,
    ArcNotFound,
    PageNotFound,
    NetNotFound,
    ObjectNotFound,
    InvalidData(String),
    CorruptedData(String),
    XmlWriterError(xml::writer::Error),
}

impl Error for PetriError {
    fn description(&self) -> &str {
        match self {
            PetriError::BipartitionViolation => "Bipartition Violation: Edges cannot lead to identical Node types. They are only allowed from places to transitions or vice versa",
            PetriError::PlaceNotFound => "Place Not Found: There is no corresponding place in the internal representation",
            PetriError::TransitionNotFound => "Transition Not Found: There is no corresponding transition in the internal representation",
            PetriError::ArcNotFound => "Arc Not Found: There is no corresponding arc in the internal representation",
            PetriError::ObjectNotFound => "Object Not Found: There is no corresponding object in the internal representation",
            PetriError::PageNotFound => "Page Not Found: Could not find (sub)page in the given path",
            PetriError::NetNotFound => "Net Not Found: Could not find the referenced petri net in the pnml document",
            PetriError::InvalidData(_) => "Invalid Data: Tried to use data in a place where it do not belong",
            PetriError::CorruptedData(_) => "Corrupted Data: There where objects in arrays there they shouldn't be",
            PetriError::XmlWriterError(error) => error.description()
        }
    }
}

impl std::fmt::Display for PetriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            PetriError::InvalidData(msg) => format!("{}: {}", self.description(), msg),
            PetriError::CorruptedData(msg) => format!("{}: {}", self.description(), msg),
            _ => format!("{}", self.description()),
        };
        write!(f, "{}", msg)
    }
}

impl std::fmt::Debug for PetriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            PetriError::InvalidData(msg) => format!("{}: {}", self.description(), msg),
            PetriError::CorruptedData(msg) => format!("{}: {}", self.description(), msg),
            _ => format!("{}", self.description()),
        };
        write!(f, "{}", msg)
    }
}

impl From<xml::writer::Error> for PetriError {
    fn from(error: xml::writer::Error) -> Self {
        PetriError::XmlWriterError(error)
    }
}
