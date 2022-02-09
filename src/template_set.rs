use crate::build_config::BuildConfig;
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
        let kind_temp = format!("{}_temp", kind);

        let file = match tman.get(&kind_fname) {
            Some(s) => s.clone(),
            None => {
                let st = config
                    .get(&kind_temp)
                    .map(|t| t.to_string())
                    .or_else(|| {
                        config
                            .get(&kind_fname)
                            .map(|d| format!("{}{{{{.page_number}}}}.svg", d))
                    })
                    .unwrap_or(format!("out/{}{{{{.page_number}}}}.svg", kind));
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

    pub fn build_page_string<'a, IT: Iterator<Item = (usize, &'a Card)>>(
        &self,
        cards: &mut IT,
        bc: &mut BuildConfig,
    ) -> anyhow::Result<Option<String>> {
        let mut cards_str = String::new();
        for i in 0..bc.dims.per_page() {
            if let Some((_cnum, c)) = cards.next() {
                let (x, y) = bc.dims.pos(i);
                let cstr = self.card.run(&[c, &bc.config], &mut bc.tman, &bc.fman)?;
                let mut map = HashMap::new();
                map.insert("current_card", TData::String(cstr));
                map.insert("current_x", TData::Float(x));
                map.insert("current_y", TData::Float(y));

                cards_str.push_str(&self.card_wrap.run(
                    &[&map, &bc.config],
                    &mut bc.tman,
                    &bc.fman,
                )?);
            } else {
                if i == 0 {
                    return Ok(None);
                }
                break;
            }
        }

        bc.config
            .insert("cards".to_string(), TData::String(cards_str));

        Ok(Some(self.page.run(
            &[&bc.config],
            &mut bc.tman,
            &bc.fman,
        )?))
    }

    ///@return Path of written file
    pub fn build_page_file<'a, IT: Iterator<Item = (usize, &'a Card)>>(
        &self,
        n: usize,
        cards: &mut IT,
        bc: &mut BuildConfig,
    ) -> anyhow::Result<Option<String>> {
        let s = match self.build_page_string(cards, bc)? {
            Some(s) => s,
            None => return Ok(None),
        };
        let path = self.file.run(
            &[&(&KV("page_number", n), &bc.config)],
            &mut bc.tman,
            &bc.fman,
        )?;
        std::fs::write(&path, s)?;
        Ok(Some(path))
    }

    pub fn build_page_files<'a, IT: Iterator<Item = (usize, &'a Card)>>(
        &self,
        mut cards: IT,
        bc: &mut BuildConfig,
    ) -> anyhow::Result<()> {
        //Get the right template files
        let mut i = 0;
        while let Some(_s) = self.build_page_file(i, &mut cards, bc)? {
            //TODO store written file names
            i += 1;
        }
        Ok(())
    }
}
