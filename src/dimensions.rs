use std::collections::HashMap;
use templito::TData;

pub struct Dimensions {
    columns: usize,
    pw: f64,
    ph: f64,
    cw: f64,
    ch: f64,
    padding: f64,
    margin: f64,
    units: String,
    max_cards: usize,
}

impl Dimensions {
    pub fn new(data: HashMap<String, TData>) -> Self {
        let pw = data
            .get("page_width")
            .and_then(|w| w.as_float())
            .unwrap_or(210.);
        let ph = data
            .get("page_height")
            .and_then(|w| w.as_float())
            .unwrap_or(297.);

        //let mut columns = data.get("columns").or_else(||

        Dimensions { columns: 1, pw, ph }
    }
}
