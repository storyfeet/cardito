use std::collections::HashMap;
use templito::TData;

pub struct Spread {
    start: f64,
    step: f64,
}

impl Spread {
    fn pos(&self, n: usize) -> f64 {
        self.start + self.step * (n as f64)
    }
}

pub struct Dimensions {
    columns: usize,
    rows: usize,
    /*pw: f64,
    ph: f64,
    cw: f64,
    ch: f64,
    padding: f64,
    margin: f64,*/
    spread_x: Spread,
    spread_y: Spread,
}

fn float_or(data: &HashMap<String, TData>, s: &str, fl: f64) -> f64 {
    data.get(s).and_then(|w| w.as_float()).unwrap_or(fl)
}

fn max_cards(page: f64, card: f64, margin: f64, padding: f64) -> usize {
    let available = page - (margin * 2. + padding);
    (available / (card + padding)) as usize
}

fn spread_cards(n: usize, page: f64, card: f64, padding: f64) -> Spread {
    let n = n as f64;
    let start = (page - n * card - (n - 1.) * padding) / 2.;
    Spread {
        start,
        step: card + padding,
    }
}

impl Dimensions {
    pub fn new(data: &HashMap<String, TData>) -> Self {
        let pw = float_or(data, "page_width", 210.);
        let ph = float_or(data, "page_height", 297.);
        let margin = float_or(data, "margin", 5.);
        let padding = float_or(data, "padding", 0.);
        let cw = float_or(data, "card_width", 45.);
        let ch = float_or(data, "card_height", 60.);
        let columns = max_cards(pw, cw, margin, padding);
        let rows = max_cards(ph, ch, margin, padding);
        let spread_x = spread_cards(columns, pw, cw, padding);
        let spread_y = spread_cards(rows, ph, ch, padding);

        //let mut columns = data.get("columns").or_else(||

        Dimensions {
            /*pw,
            ph,
            margin,
            padding,
            cw,
            ch,*/
            rows,
            columns,
            spread_x,
            spread_y,
        }
    }

    pub fn pos(&self, n: usize) -> (f64, f64) {
        let x = n % self.columns;
        let y = n / self.columns;
        (self.spread_x.pos(x), self.spread_y.pos(y))
    }

    pub fn per_page(&self) -> usize {
        self.rows * self.columns
    }
}
