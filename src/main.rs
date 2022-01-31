use card_format::Card;
use clap::*;
use err_tools::*;
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
        print_funcs(sub, &fman);
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

    let mut prim_tree = templito::TreeTemplate::from_str(&primary)?;
    let (_, config) = prim_tree.run_exp(&[], &mut tm, fman)?;

    let cards = if let Some(card_path) = config.get("card_files") {
        read_cards(card_path)?
    } else if let Some(card_string) = config.get("card_string") {
        card_format::parse_cards(&card_string.to_string())?
    } else {
        return e_str("No Cards supplied: use 'card_files' or 'card_string'");
    };

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
        _ => unimplemented! {},
    }
}
