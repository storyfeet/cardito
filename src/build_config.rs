use crate::dimensions::Dimensions;
use clap::*;
use err_tools::*;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use templito::{
    func_man::BasicFuncs, temp_man::BasicTemps as TMan, TData, TreeTemplate, WithFuncs,
};

pub struct BuildConfig {
    pub config: HashMap<String, TData>,
    pub tman: TMan,
    pub fman: BasicFuncs,
    pub dims: Dimensions,
}

pub fn func_man(clp: &ArgMatches) -> BasicFuncs {
    let mut fman = BasicFuncs::new().with_defaults();

    if clp.is_present("trusted") {
        fman = fman.with_exec().with_free_files();
    }
    fman
}

impl BuildConfig {
    pub fn try_new(clp: &ArgMatches, fman: BasicFuncs) -> anyhow::Result<Self> {
        let mut tman = TMan::new();

        let mut init_data = HashMap::new();

        let primary: String = match clp.value_of("file") {
            Some(fname) => {
                init_data.insert(
                    "template_path".to_string(),
                    TData::String(fname.to_string()),
                );
                std::fs::read_to_string(fname)?
            }
            None => {
                let mut s = String::new();
                std::io::stdin()
                    .read_to_string(&mut s)
                    .e_str("No file and nothing in stdin")?;
                s
            }
        };

        if let Some(mut vals) = clp.values_of("vars") {
            while let Some(k) = vals.next() {
                match vals.next() {
                    Some(v) => {
                        let nval =
                            TData::from_str(v).unwrap_or_else(|_| TData::String(v.to_string()));
                        init_data.insert(k.to_string(), nval);
                    }
                    None => return e_str("for --vars keys must have values").into(),
                }
            }
        }

        let prim_tree = templito::TreeTemplate::from_str(&primary)?;

        let (_, mut config) = prim_tree.run_exp(&[&init_data], &mut tman, &fman)?;
        for (k, v) in init_data {
            config.insert(k, v);
        }

        //Add data from clap to config
        if let Some(s) = clp.value_of("cards") {
            config.insert("card_files".to_string(), TData::String(s.to_string()));
        }
        if let Some(s) = clp.value_of("fpath") {
            config.insert("front_path".to_string(), TData::String(s.to_string()));
        }
        if let Some(s) = clp.value_of("bpath") {
            config.insert("back_path".to_string(), TData::String(s.to_string()));
        }
        if let Some(s) = clp.value_of("fpath_temp") {
            config.insert(
                "front_temp".to_string(),
                TData::Template(TreeTemplate::from_str(s)?),
            );
        }
        if let Some(s) = clp.value_of("bpath_temp") {
            config.insert(
                "back_temp".to_string(),
                TData::Template(TreeTemplate::from_str(s)?),
            );
        }

        //Run function imports

        if let Some(s) = c

        //finalize other parts
        let dims = Dimensions::new(&config);

        Ok(BuildConfig {
            tman,
            fman,
            config,
            dims,
        })
    }
    pub fn set_reverse(&mut self, b: bool) {
        self.dims.reverse = b;
    }
}
