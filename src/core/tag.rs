use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::{
    borrow::Cow,
    cell::UnsafeCell,
    fmt::{Debug, Display},
    rc::Rc,
};

pub trait DicomTag: Debug + Display {
    fn name(&self) -> String;
    fn tag(&self) -> (u16, u16);
    fn vr(&self) -> VisualRepresentation;
    fn group(&self) -> u16;
    fn element(&self) -> Option<u16>;
    fn is_deprecated(&self) -> bool;
    fn multiplicity(&self) -> &str;
}

pub enum DicomValue<'a> {
    String(&'a dyn ToString),
    Object(Rc<dyn DicomTag>),
    ObjectVec(Vec<Rc<dyn DicomTag>>),
}

#[derive(Debug, Clone)]
pub enum VisualRepresentation {
    AE(Cow<'static, str>),     // Application Entity
    AS(Cow<'static, str>),     // Age String
    AT(Cow<'static, str>),     // Attribute Tag
    CS(Cow<'static, str>),     // Code String
    DA(NaiveDate),             // Date
    DS(Cow<'static, str>),     // Decimal String
    DT(NaiveDateTime),         // DateTime
    FL(f32),                   // Floating Point Single
    FD(f64),                   // Floating Point Double
    IS(Cow<'static, str>),     // Integer String
    LO(Cow<'static, str>),     // Long String
    LT(Cow<'static, str>),     // Long Text
    OB(Vec<u8>),               // Other Byte String
    OD(Vec<f64>),              // Other Double String
    OF(Vec<f32>),              // Other Float String
    OL(Vec<u32>),              // Other Long String
    OV(Vec<i64>),              // Other Very Long String
    OW(Vec<u16>),              // Other Word String
    PN(Cow<'static, str>),     // Person Name
    SH(Cow<'static, str>),     // Short String
    SL(i32),                   // Signed Long
    SQ(Vec<Rc<dyn DicomTag>>), // Sequence of Items
    SS(i16),                   // Signed Short
    ST(Cow<'static, str>),     // Short Text
    SV(i64),                   // Signed Very Long
    TM(NaiveTime),             // Time
    UC(Cow<'static, str>),     // Unlimited Characters
    UI(Cow<'static, str>),     // Unique Identifier (UID)
    UL(u32),                   // Unsigned Long
    UN(Vec<u8>),               // Unknown
    UR(Cow<'static, str>),     // Universal Resource Identifier
    US(u16),                   // Unsigned Short
    UT(Cow<'static, str>),     // Unlimited Text
}

impl !Send for VisualRepresentation {}
impl !Sync for VisualRepresentation {}

impl VisualRepresentation {
    pub fn from_string(vr: &str, value: &str) -> Self {
        match vr {
            "AE" => VisualRepresentation::AE(value.to_string().into()),
            "AS" => VisualRepresentation::AS(value.to_string().into()),
            "AT" => VisualRepresentation::AT(value.to_string().into()),
            "CS" => VisualRepresentation::CS(value.to_string().into()),
            "DA" => VisualRepresentation::DA(NaiveDate::parse_from_str(value, "%Y%m%d").unwrap()),
            "DS" => VisualRepresentation::DS(value.to_string().into()),
            "DT" => VisualRepresentation::DT(
                NaiveDateTime::parse_from_str(value, "%Y%m%d%H%M%S%.6f").unwrap(),
            ),
            "FL" => VisualRepresentation::FL(value.parse().unwrap()),
            "FD" => VisualRepresentation::FD(value.parse().unwrap()),
            "IS" => VisualRepresentation::IS(value.to_string().into()),
            "LO" => VisualRepresentation::LO(value.to_string().into()),
            "LT" => VisualRepresentation::LT(value.to_string().into()),
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
            "PN" => VisualRepresentation::PN(value.to_string().into()),
            "SH" => VisualRepresentation::SH(value.to_string().into()),
            "SL" => VisualRepresentation::SL(value.parse().unwrap()),
            "SQ" => VisualRepresentation::SQ(vec![]),
            "SS" => VisualRepresentation::SS(value.parse().unwrap()),
            "ST" => VisualRepresentation::ST(value.to_string().into()),
            "SV" => VisualRepresentation::SV(value.parse().unwrap()),
            "TM" => {
                VisualRepresentation::TM(NaiveTime::parse_from_str(value, "%H%M%S%.6f").unwrap())
            }
            "UC" => VisualRepresentation::UC(value.to_string().into()),
            "UI" => VisualRepresentation::UI(value.to_string().into()),
            "UL" => VisualRepresentation::UL(value.parse().unwrap()),
            "UN" => VisualRepresentation::UN(value.as_bytes().to_vec()),
            "UR" => VisualRepresentation::UR(value.to_string().into()),
            "US" => VisualRepresentation::US(value.parse().unwrap()),
            "UT" => VisualRepresentation::UT(value.to_string().into()),
            _ => VisualRepresentation::UN(value.as_bytes().to_vec()),
        }
    }

    pub fn new(vr: &str) -> Self {
        match vr {
            "AE" => VisualRepresentation::AE(Cow::default()),
            "AS" => VisualRepresentation::AS(Cow::default()),
            "AT" => VisualRepresentation::AT(Cow::default()),
            "CS" => VisualRepresentation::CS(Cow::default()),
            "DA" => VisualRepresentation::DA(NaiveDate::default()),
            "DS" => VisualRepresentation::DS(Cow::default()),
            "DT" => VisualRepresentation::DT(NaiveDateTime::default()),
            "FL" => VisualRepresentation::FL(0.0),
            "FD" => VisualRepresentation::FD(0.0),
            "IS" => VisualRepresentation::IS(Cow::default()),
            "LO" => VisualRepresentation::LO(Cow::default()),
            "LT" => VisualRepresentation::LT(Cow::default()),
            "OB" => VisualRepresentation::OB(vec![]),
            "OD" => VisualRepresentation::OD(vec![]),
            "OF" => VisualRepresentation::OF(vec![]),
            "OL" => VisualRepresentation::OL(vec![]),
            "OV" => VisualRepresentation::OV(vec![]),
            "OW" => VisualRepresentation::OW(vec![]),
            "PN" => VisualRepresentation::PN(Cow::default()),
            "SH" => VisualRepresentation::SH(Cow::default()),
            "SL" => VisualRepresentation::SL(0),
            "SQ" => VisualRepresentation::SQ(vec![]),
            "SS" => VisualRepresentation::SS(0),
            "ST" => VisualRepresentation::ST(Cow::default()),
            "SV" => VisualRepresentation::SV(0),
            "TM" => VisualRepresentation::TM(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            "UC" => VisualRepresentation::UC(Cow::default()),
            "UI" => VisualRepresentation::UI(Cow::default()),
            "UL" => VisualRepresentation::UL(0),
            "UN" => VisualRepresentation::UN(vec![]),
            "UR" => VisualRepresentation::UR(Cow::default()),
            "US" => VisualRepresentation::US(0),
            "UT" => VisualRepresentation::UT(Cow::default()),
            _ => VisualRepresentation::UN(vec![]),
        }
    }

    pub fn set(&self, value: DicomValue) -> &Self {
        // SAFETY: This is safe because the inner value is set based on the type of the VisualRepresentation
        unsafe { self.set_inner(value) }
    }

    unsafe fn set_inner(&self, value: DicomValue) -> &Self {
        let mutable_self = UnsafeCell::new(self.clone());
        unsafe {
            match &mut *mutable_self.get() {
                VisualRepresentation::AE(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::AS(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::AT(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::CS(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::DA(v) => {
                    *v = NaiveDate::parse_from_str(&value.to_string(), "%Y%m%d").unwrap();
                }
                VisualRepresentation::DS(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::DT(v) => {
                    *v = NaiveDateTime::parse_from_str(&value.to_string(), "%Y%m%d%H%M%S%.6f")
                        .unwrap();
                }
                VisualRepresentation::FL(v) => {
                    *v = value.to_string().parse().unwrap();
                }
                VisualRepresentation::FD(v) => {
                    *v = value.to_string().parse().unwrap();
                }
                VisualRepresentation::IS(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::LO(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::LT(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::OB(v) => {
                    *v = value.to_string().as_bytes().to_vec();
                }
                VisualRepresentation::OD(v) => {
                    *v = value
                        .to_string()
                        .split_whitespace()
                        .map(|s| s.parse::<f64>().unwrap())
                        .collect::<Vec<_>>();
                }
                VisualRepresentation::OF(v) => {
                    *v = value
                        .to_string()
                        .split_whitespace()
                        .map(|s| s.parse::<f32>().unwrap())
                        .collect::<Vec<_>>();
                }
                VisualRepresentation::OL(v) => {
                    *v = value
                        .to_string()
                        .split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>();
                }
                VisualRepresentation::OV(v) => {
                    *v = value
                        .to_string()
                        .split_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                }
                VisualRepresentation::OW(v) => {
                    *v = value
                        .to_string()
                        .split_whitespace()
                        .map(|s| s.parse::<u16>().unwrap())
                        .collect::<Vec<_>>();
                }
                VisualRepresentation::PN(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::SH(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::SL(v) => {
                    *v = value.to_string().parse().unwrap();
                }
                VisualRepresentation::SQ(v) => {
                    *v = value.object_vec().into_iter().collect();
                }
                VisualRepresentation::SS(v) => {
                    *v = value.to_string().parse().unwrap();
                }
                VisualRepresentation::ST(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::SV(v) => {
                    *v = value.to_string().parse().unwrap();
                }
                VisualRepresentation::TM(v) => {
                    *v = NaiveTime::parse_from_str(&value.to_string(), "%H%M%S%.6f").unwrap();
                }
                VisualRepresentation::UC(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::UI(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::UL(v) => {
                    *v = value.to_string().parse().unwrap();
                }
                VisualRepresentation::UN(v) => {
                    *v = value.to_string().as_bytes().to_vec();
                }
                VisualRepresentation::UR(v) => {
                    *v = value.to_string().into();
                }
                VisualRepresentation::US(v) => {
                    *v = value.to_string().parse().unwrap();
                }
                VisualRepresentation::UT(v) => {
                    *v = value.to_string().into();
                }
            };
        }

        self
    }
}

impl Display for DicomValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DicomValue::String(v) => write!(f, "{}", v.to_string()),
            DicomValue::Object(v) => write!(f, "{:#?}", v),
            DicomValue::ObjectVec(v) => {
                write!(f, "{:#?}", v)
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
