mod dimensions;
mod templates;

use card_format::Card;
use clap::*;
use err_tools::*;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use templito::func_man::{BasicFuncs, FuncManager, WithFuncs};
use templito::TData;

fn main() -> anyhow::Result<()> {
    let clp = App::new("Cardito")
        .version(crate_version!())
        .about("Build Cards into a pdf from a templito template")
        .author("Matthew Stoodley")
        .subcommand(
            App::new("funcs")
                .about("print available funcs")
                .args(&[arg!(-f --filter [filter] "A string to filter commands by")]),
        )
        .subcommand(App::new("build").about("build cards to svg").args(&[
            arg!(-f --file [file_name] "The primary template file"),
            arg!(-t --templates [templates] "The file or folder where utility templates are found"),
            arg!(-c --cards [cards] "The card file"),
        ]))
        .args(&[arg!(--trusted "Give the templates ability to execute functions and read and write files")])
        .get_matches();

    let mut fman = BasicFuncs::new().with_defaults();
    if clp.is_present("trusted") {
        fman = fman.with_exec().with_free_files();
    }

    if let Some(sub) = clp.subcommand_matches("funcs") {
        print_funcs(sub, &fman);
    }

    if let Some(sub) = clp.subcommand_matches("build") {
        build_cards(sub, &fman)?;
    }
    Ok(())
}

pub fn print_funcs(clp: &ArgMatches, fman: &BasicFuncs) {
    if let Some(f) = clp.value_of("filter") {
        fman.print_filter(f);
    } else {
        fman.print_all();
    }
}

pub fn build_cards(clp: &ArgMatches, fman: &BasicFuncs) -> anyhow::Result<()> {
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
    let mut tm = templito::temp_man::BasicTemps::new();

    let prim_tree = templito::TreeTemplate::from_str(&primary)?;
    let (_, mut config) = prim_tree.run_exp(&[], &mut tm, fman)?;

    let cards = if let Some(card_path) = config.get("card_files") {
        read_cards(card_path)?
    } else if let Some(card_string) = config.get("card_string") {
        card_format::parse_cards(&card_string.to_string())?
    } else {
        return e_str("No Cards supplied: use 'card_files' or 'card_string'");
    };

    let ctemplate = tm.get("card").e_str("No card template provided")?.clone();

    let card_wrap = tm.get("card_wrap").map(|c| c.clone()).unwrap_or_else(|| {
        templito::TreeTemplate::from_str(templates::CARD_WRAP)
            .expect("Builtin Templates should work (CARD_WRAP)")
    });

    let mut cards_str = String::new();
    for c in cards {
        let cstr = ctemplate.run(&[&c, &config], &mut tm, fman)?;
        let mut map = HashMap::new();
        map.insert("current_card", TData::String(cstr));
        map.insert("current_x", TData::Int(3));
        map.insert("current_y", TData::Int(4));

        cards_str.push_str(&card_wrap.run(&[&map, &config], &mut tm, fman)?);
    }

    config.insert("cards".to_string(), TData::String(cards_str));

    let page_template = tm.get("page").map(|c| c.clone()).unwrap_or_else(|| {
        templito::TreeTemplate::from_str(templates::PAGE_TEMPLATE)
            .expect("Builtin templates should work (PAGE_TEMPLATE)")
    });

    let page_result = page_template.run(&[&config], &mut tm, fman)?;

    println!("{}", page_result);

    Ok(())
}

pub fn read_cards(data: &TData) -> anyhow::Result<Vec<Card>> {
    match data {
        TData::List(l) => {
            let mut res = Vec::new();
            for a in l {
                res.extend(read_cards(a)?);
            }
            Ok(res)
        }
        TData::String(fname) => {
            let f = std::fs::read_to_string(fname)?;
            let cards = card_format::parse_cards(&f)?;
            Ok(cards)
        }
        _ => unimplemented! {},
    }
}
