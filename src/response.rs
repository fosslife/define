use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct WelcomeElement {
    pub(crate) meta: Meta,
    pub(crate) hwi: Hwi,
    pub(crate) fl: String,
    // pub(crate) def: Vec<Def>,
    // pub(crate) quotes: Vec<Quote>,
    pub(crate) et: Option<Vec<Vec<String>>>,
    // pub(crate) date: String,
    pub(crate) shortdef: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DtClass {
    pub(crate) t: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hwi {
    pub(crate) hw: String,
    pub(crate) prs: Option<Vec<Pr>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pr {
    pub(crate) mw: String,
    // pub(crate) sound: Sound,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub(crate) id: String,
    pub(crate) uuid: String,
    pub(crate) sort: String,
    pub(crate) src: String,
    pub(crate) section: String,
    pub(crate) stems: Vec<String>,
    pub(crate) offensive: bool,
}
