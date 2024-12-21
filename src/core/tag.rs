use std::{fmt::{Debug, Display}, rc::Rc};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub trait DicomTag: Debug + ToString {
    fn name(&self) -> String;
    fn code(&self) -> (u16, u16);
    fn vr(&mut self) -> &mut VisualRepresentation;
    fn group(&self) -> u16;
    fn element(&self) -> Option<u16>;
    fn is_deprecated(&self) -> bool;
}

pub enum DicomValue<'a> {
    String(&'a dyn ToString),
    Object(Rc<dyn DicomTag>),
    ObjectVec(Vec<Rc<dyn DicomTag>>),
}

#[derive(Debug)]
pub enum VisualRepresentation {
    AE(String),                 // Application Entity
    AS(String),                 // Age String
    AT(String),                 // Attribute Tag
    CS(String),                 // Code String
    DA(NaiveDate),              // Date
    DS(String),                 // Decimal String
    DT(NaiveDateTime),          // DateTime
    FL(f32),                    // Floating Point Single
    FD(f64),                    // Floating Point Double
    IS(String),                 // Integer String
    LO(String),                 // Long String
    LT(String),                 // Long Text
    OB(Vec<u8>),                // Other Byte String
    OD(Vec<f64>),               // Other Double String
    OF(Vec<f32>),               // Other Float String
    OL(Vec<u32>),               // Other Long String
    OV(Vec<i64>),               // Other Very Long String
    OW(Vec<u16>),               // Other Word String
    PN(String),                 // Person Name
    SH(String),                 // Short String
    SL(i32),                    // Signed Long
    SQ(Vec<Rc<dyn DicomTag>>),  // Sequence of Items
    SS(i16),                    // Signed Short
    ST(String),                 // Short Text
    SV(i64),                    // Signed Very Long
    TM(NaiveTime),              // Time
    UC(String),                 // Unlimited Characters
    UI(String),                 // Unique Identifier (UID)
    UL(u32),                    // Unsigned Long
    UN(Vec<u8>),                // Unknown
    UR(String),                 // Universal Resource Identifier
    US(u16),                    // Unsigned Short
    UT(String),                 // Unlimited Text
}

impl VisualRepresentation {
    pub fn from_string(vr: &str, value: &str) -> Self {
        match vr {
            "AE" => VisualRepresentation::AE(value.to_string()),
            "AS" => VisualRepresentation::AS(value.to_string()),
            "AT" => VisualRepresentation::AT(value.to_string()),
            "CS" => VisualRepresentation::CS(value.to_string()),
            "DA" => VisualRepresentation::DA(NaiveDate::parse_from_str(value, "%Y%m%d").unwrap()),
            "DS" => VisualRepresentation::DS(value.to_string()),
            "DT" => VisualRepresentation::DT(NaiveDateTime::parse_from_str(value, "%Y%m%d%H%M%S%.6f").unwrap()),
            "FL" => VisualRepresentation::FL(value.parse().unwrap()),
            "FD" => VisualRepresentation::FD(value.parse().unwrap()),
            "IS" => VisualRepresentation::IS(value.to_string()),
            "LO" => VisualRepresentation::LO(value.to_string()),
            "LT" => VisualRepresentation::LT(value.to_string()),
            "OB" => VisualRepresentation::OB(value.as_bytes().to_vec()),
            "OD" => VisualRepresentation::OD(
                value
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            ),
            "OF" => VisualRepresentation::OF(
                value
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            ),
            "OL" => VisualRepresentation::OL(
                value
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            ),
            "OV" => VisualRepresentation::OV(
                value
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            ),
            "OW" => VisualRepresentation::OW(
                value
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            ),
            "PN" => VisualRepresentation::PN(value.to_string()),
            "SH" => VisualRepresentation::SH(value.to_string()),
            "SL" => VisualRepresentation::SL(value.parse().unwrap()),
            "SQ" => VisualRepresentation::SQ(vec![]),
            "SS" => VisualRepresentation::SS(value.parse().unwrap()),
            "ST" => VisualRepresentation::ST(value.to_string()),
            "SV" => VisualRepresentation::SV(value.parse().unwrap()),
            "TM" => VisualRepresentation::TM(NaiveTime::parse_from_str(value, "%H%M%S%.6f").unwrap()),
            "UC" => VisualRepresentation::UC(value.to_string()),
            "UI" => VisualRepresentation::UI(value.to_string()),
            "UL" => VisualRepresentation::UL(value.parse().unwrap()),
            "UN" => VisualRepresentation::UN(value.as_bytes().to_vec()),
            "UR" => VisualRepresentation::UR(value.to_string()),
            "US" => VisualRepresentation::US(value.parse().unwrap()),
            "UT" => VisualRepresentation::UT(value.to_string()),
            _ => VisualRepresentation::UN(value.as_bytes().to_vec()),
        }
    }

    pub fn new(vr: &str) -> Self {
        match vr {
            "AE" => VisualRepresentation::AE(String::new()),
            "AS" => VisualRepresentation::AS(String::new()),
            "AT" => VisualRepresentation::AT(String::new()),
            "CS" => VisualRepresentation::CS(String::new()),
            "DA" => VisualRepresentation::DA(NaiveDate::default()),
            "DS" => VisualRepresentation::DS(String::new()),
            "DT" => VisualRepresentation::DT(NaiveDateTime::default()),
            "FL" => VisualRepresentation::FL(0.0),
            "FD" => VisualRepresentation::FD(0.0),
            "IS" => VisualRepresentation::IS(String::new()),
            "LO" => VisualRepresentation::LO(String::new()),
            "LT" => VisualRepresentation::LT(String::new()),
            "OB" => VisualRepresentation::OB(vec![]),
            "OD" => VisualRepresentation::OD(vec![]),
            "OF" => VisualRepresentation::OF(vec![]),
            "OL" => VisualRepresentation::OL(vec![]),
            "OV" => VisualRepresentation::OV(vec![]),
            "OW" => VisualRepresentation::OW(vec![]),
            "PN" => VisualRepresentation::PN(String::new()),
            "SH" => VisualRepresentation::SH(String::new()),
            "SL" => VisualRepresentation::SL(0),
            "SQ" => VisualRepresentation::SQ(vec![]),
            "SS" => VisualRepresentation::SS(0),
            "ST" => VisualRepresentation::ST(String::new()),
            "SV" => VisualRepresentation::SV(0),
            "TM" => VisualRepresentation::TM(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            "UC" => VisualRepresentation::UC(String::new()),
            "UI" => VisualRepresentation::UI(String::new()),
            "UL" => VisualRepresentation::UL(0),
            "UN" => VisualRepresentation::UN(vec![]),
            "UR" => VisualRepresentation::UR(String::new()),
            "US" => VisualRepresentation::US(0),
            "UT" => VisualRepresentation::UT(String::new()),
            _ => VisualRepresentation::UN(vec![]),
        }
    }

    pub fn set<T>(&mut self, value: DicomValue) -> &mut Self
    {
        match self {
            VisualRepresentation::AE(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::AS(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::AT(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::CS(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::DA(v) => {
                *v = NaiveDate::parse_from_str(&value.to_string(), "%Y%m%d").unwrap();
                self
            }
            VisualRepresentation::DS(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::DT(v) => {
                *v = NaiveDateTime::parse_from_str(&value.to_string(), "%Y%m%d%H%M%S%.6f").unwrap();
                self
            }
            VisualRepresentation::FL(v) => {
                *v = value.to_string().parse().unwrap();
                self
            }
            VisualRepresentation::FD(v) => {
                *v = value.to_string().parse().unwrap();
                self
            }
            VisualRepresentation::IS(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::LO(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::LT(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::OB(v) => {
                *v = value.to_string().as_bytes().to_vec();
                self
            }
            VisualRepresentation::OD(v) => {
                *v = value
                    .to_string()
                    .split_whitespace()
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
                self
            }
            VisualRepresentation::OF(v) => {
                *v = value
                    .to_string()
                    .split_whitespace()
                    .map(|s| s.parse::<f32>().unwrap())
                    .collect::<Vec<_>>();
                self
            }
            VisualRepresentation::OL(v) => {
                *v = value
                    .to_string()
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                self
            }
            VisualRepresentation::OV(v) => {
                *v = value
                    .to_string()
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                self
            }
            VisualRepresentation::OW(v) => {
                *v = value
                    .to_string()
                    .split_whitespace()
                    .map(|s| s.parse::<u16>().unwrap())
                    .collect::<Vec<_>>();
                self
            }
            VisualRepresentation::PN(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::SH(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::SL(v) => {
                *v = value.to_string().parse().unwrap();
                self
            }
            VisualRepresentation::SQ(v) => {
                *v = value.object_vec().into_iter().collect();
                self
            }
            VisualRepresentation::SS(v) => {
                *v = value.to_string().parse().unwrap();
                self
            }
            VisualRepresentation::ST(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::SV(v) => {
                *v = value.to_string().parse().unwrap();
                self
            }
            VisualRepresentation::TM(v) => {
                *v = NaiveTime::parse_from_str(&value.to_string(), "%H%M%S%.6f").unwrap();
                self
            }
            VisualRepresentation::UC(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::UI(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::UL(v) => {
                *v = value.to_string().parse().unwrap();
                self
            }
            VisualRepresentation::UN(v) => {
                *v = value.to_string().as_bytes().to_vec();
                self
            }
            VisualRepresentation::UR(v) => {
                *v = value.to_string();
                self
            }
            VisualRepresentation::US(v) => {
                *v = value.to_string().parse().unwrap();
                self
            }
            VisualRepresentation::UT(v) => {
                *v = value.to_string();
                self
            }
        }
    }
}

impl Display for DicomValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DicomValue::String(v) => write!(f, "{}", v.to_string()),
            DicomValue::Object(v) => write!(f, "{}", v.to_string()),
            DicomValue::ObjectVec(v) => {
                let mut s = String::new();
                for obj in v {
                    s.push_str(&obj.to_string());
                }
                write!(f, "{}", s)
            }
        }
    }
}

impl DicomValue<'_> {
    pub fn object(&self) -> Option<Rc<dyn DicomTag>> {
        match self {
            DicomValue::Object(v) => Some(Rc::clone(v)),
            _ => None,
        }
    }

    pub fn object_vec(&self) -> Vec<Rc<dyn DicomTag>> {
        match self {
            DicomValue::ObjectVec(v) => v.iter().map(|obj| Rc::clone(obj)).collect(),
            _ => vec![],
        }
    }
}

include!("generated.rs");
