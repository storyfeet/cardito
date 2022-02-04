use crate::build_config::BuildConfig;
use crate::dimensions::Dimensions;
use crate::templates;
use card_format::Card;
use std::collections::HashMap;
use std::str::FromStr;
use templito::{temp_man::BasicTemps as TMan, TData, TreeTemplate, KV};

pub struct TemplateSet {
    pub page: TreeTemplate,
    pub card: TreeTemplate,
    pub card_wrap: TreeTemplate,
    pub file: TreeTemplate,
}

impl TemplateSet {
    pub fn try_new(
        kind: &str,
        config: &HashMap<String, TData>,
        tman: &TMan,
    ) -> anyhow::Result<Option<Self>> {
        let card = match tman.get(kind) {
            Some(c) => c.clone(),
            None => return Ok(None),
        };
        let kind_fname = format!("{}_path", kind);

        let file = match tman.get(&kind_fname) {
            Some(s) => s.clone(),
            None => {
                let st = config
                    .get(&kind_fname)
                    .map(|d| format!("{}{{{{}}}}.svg", d))
                    .unwrap_or(format!("out/{}{{{{$n}}}}.svg", kind));
                TreeTemplate::from_str(&st)?
            }
        };

        let page = tman.get("page").map(|c| c.clone()).unwrap_or_else(|| {
            templito::TreeTemplate::from_str(templates::PAGE_TEMPLATE)
                .expect("Builtin templates should work (PAGE_TEMPLATE)")
        });
        let card_wrap = tman.get("card_wrap").map(|c| c.clone()).unwrap_or_else(|| {
            templito::TreeTemplate::from_str(templates::CARD_WRAP)
                .expect("Builtin Templates should work (CARD_WRAP)")
        });

        Ok(Some(TemplateSet {
            card,
            file,
            page,
            card_wrap,
        }))
    }

    pub fn build_page_string(
        &self,
        cards: &[Card],
        bc: &mut BuildConfig,
    ) -> anyhow::Result<String> {
        let mut cards_str = String::new();
        for (i, c) in cards.into_iter().enumerate() {
            let (x, y) = bc.dims.pos(i);
            let cstr = self.card.run(&[c, &bc.config], &mut bc.tman, &bc.fman)?;
            let mut map = HashMap::new();
            map.insert("current_card", TData::String(cstr));
            map.insert("current_x", TData::Float(x));
            map.insert("current_y", TData::Float(y));

            cards_str.push_str(
                &self
                    .card_wrap
                    .run(&[&map, &bc.config], &mut bc.tman, &bc.fman)?,
            );
        }

        bc.config
            .insert("cards".to_string(), TData::String(cards_str));

        Ok(self.page.run(&[&bc.config], &mut bc.tman, &bc.fman)?)
    }

    ///@return Path of written file
    pub fn build_page_file(
        &self,
        n: usize,
        cards: &[Card],
        bc: &mut BuildConfig,
    ) -> anyhow::Result<String> {
        let s = self.build_page_string(cards, bc)?;
        let path = self.file.run(
            &[&(&KV("page_number", n), &bc.config)],
            &mut bc.tman,
            &bc.fman,
        )?;
        std::fs::write(&path, s)?;
        Ok(path)
    }

    pub fn build_page_files(&self, cards: &[Card], bc: &mut BuildConfig) -> anyhow::Result<()> {
        //Get the right template files
        let dims = Dimensions::new(&bc.config);
        let per_page = dims.per_page();
        let pages = ((cards.len() - 1) / per_page) + 1;
        for i in 0..pages {
            self.build_page_file(i, &cards[i * per_page..], bc)?;
        }
        Ok(())
    }
}
