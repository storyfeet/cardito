use crate::dimensions::Dimensions;
use clap::*;
use err_tools::*;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use templito::{func_man::BasicFuncs, temp_man::BasicTemps as TMan, TData, WithFuncs};

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

        let primary: String = match clp.value_of("file") {
            Some(fname) => std::fs::read_to_string(fname)?,
            None => {
                let mut s = String::new();
                std::io::stdin()
                    .read_to_string(&mut s)
                    .e_str("No file and nothing in stdin")?;
                s
            }
        };
        let prim_tree = templito::TreeTemplate::from_str(&primary)?;

        let (_, mut config) = prim_tree.run_exp(&[], &mut tman, &fman)?;

        //Add data from clap to config
        if let Some(s) = clp.value_of("cards") {
            config.insert("card_files".to_string(), TData::String(s.to_string()));
        }

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
