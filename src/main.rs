mod build_config;
mod dimensions;
mod spread;
mod template_set;
mod templates;

use card_format::Card;
use clap::*;
use err_tools::*;
use spread::SpreadIter;
use template_set::TemplateSet;
use templito::func_man::{BasicFuncs, FuncManager as FMan};
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
            arg!(-o --output [out_file] "The file to write the output to"),
        ]))
        .args(&[arg!(--trusted "Give the templates ability to execute functions and read and write files")])
        .get_matches();

    let fman = build_config::func_man(&clp);

    if let Some(sub) = clp.subcommand_matches("funcs") {
        print_funcs(sub, &fman);
    }

    if let Some(sub) = clp.subcommand_matches("build") {
        build_cards(sub, fman)?;
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

pub fn build_cards(clp: &ArgMatches, fman: BasicFuncs) -> anyhow::Result<()> {
    let mut bc = build_config::BuildConfig::try_new(clp, fman)?;
    let cards = if let Some(card_path) = bc.config.get("card_files") {
        read_cards(card_path)?
    } else if let Some(card_string) = bc.config.get("card_string") {
        card_format::parse_cards(&card_string.to_string())?
    } else {
        return e_str("No Cards supplied: use 'card_files' or 'card_string'");
    };

    //todo spread cards

    let mut done = false;
    if let Some(tset) = TemplateSet::try_new("front", &bc.config, &mut bc.tman)? {
        tset.build_page_files(SpreadIter::new(&cards).enumerate(), &mut bc)?;
        done = true;
    }
    if let Some(tset) = TemplateSet::try_new("back", &bc.config, &mut bc.tman)? {
        bc.set_reverse(true);
        tset.build_page_files(SpreadIter::new(&cards).enumerate(), &mut bc)?;
        done = true;
    }

    if !done {
        println!(r#"Nothing to make please add a global template for either "front" or "back""#);
        println!(r#"Hint "{{{{@global front}}}}...{{{{/global}}}}"#);
    }

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
        _ => e_str("Cards must be either a filename or list thereof"),
    }
}
