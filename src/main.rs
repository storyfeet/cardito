mod build_config;
mod dimensions;
mod imports;
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
    let clp = Command::new("Cardito")
        .version(crate_version!())
        .about("Build Cards into a pdf from a templito template")
        .author("Matthew Stoodley")
        .subcommand(
            Command::new("funcs")
                .about("print available funcs")
                .args(&[arg!(-f --filter [filter] "A string to filter commands by")]),
        )
        .subcommand(
            Command::new("keywords")
                .about("explain keywords and their usage in cardito")
        )
        .subcommand(
            Command::new("init")
                .about("Create a new basic template file and dummy cards")
                .args(&[
                    arg!(-f --folder [folder] "The folder to create the files in"),
                    arg!(-t --temp [temp] "The template's name"),
                    arg!(-c --cards [card_file] "The name of the card file" ),
                ])
        )
        .subcommand(Command::new("build").about("build cards to svg").args(&[
            arg!(-f --file [file_name] "The primary template file"),
            arg!(-t --templates [templates] "The file or folder where utility templates are found"),
            arg!(-c --cards [cards] "The card file"),
            //arg!(-o --output [out_file] "The file to write the output to"),
            arg!(-v --vars [vars] ... r#"K V pairs of values interpreted as Template data eg name:"pete""#).max_values(100),
            arg!(--fpath [fpath] "The base path for card fronts to go ({{.page_number}}.svg will be appended)"),
            arg!(--bpath [fpath] "The base path for card backs to go ({{.page_number}}.svg will be appended)"),
            arg!(--fpath_temp [fpath_temp] "A Template describing where the front files will output"),
            arg!(--bpath_temp [fpath_temp] "A Template describing where the back files will output"),
            arg!(-i --imports [imports] "Location of any templates to with global functions").max_values(100),
        ]))
        .subcommand(Command::new("table").about("Print table of key values").args(&[
                arg!(<card_file> "The name of the card file" ),
        ]))
        .args(&[arg!(--trusted "Give the templates ability to execute functions and read and write files")])
        .get_matches();

    if let Some(_sub) = clp.subcommand_matches("keywords") {
        println!("{}", std::include_str!("text/keywords.md"));
    }

    if let Some(sub) = clp.subcommand_matches("init") {
        init(sub)?;
        return Ok(());
    }
    if let Some(sub) = clp.subcommand_matches("table") {
        build_table(sub)?;
    }

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
    let mut flist = Vec::new();
    if let Some(tset) = TemplateSet::try_new("front", &bc.config, &mut bc.tman)? {
        flist = tset.build_page_files(SpreadIter::new(&cards).enumerate(), &mut bc)?;
        done = true;
    }

    let mut blist = Vec::new();
    if let Some(tset) = TemplateSet::try_new("back", &bc.config, &mut bc.tman)? {
        bc.set_reverse(true);
        blist = tset.build_page_files(SpreadIter::new(&cards).enumerate(), &mut bc)?;
        done = true;
    }

    for f in itertools::interleave(flist, blist) {
        println!("{}", f);
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
            let f = std::fs::read_to_string(fname)
                .e_string(format!("Could not read file {}", fname))?;
            let cards = card_format::parse_cards(&f)?;
            Ok(cards)
        }
        _ => e_str("Cards must be either a filename or list thereof"),
    }
}

pub fn init(clp: &ArgMatches) -> anyhow::Result<()> {
    let folder = clp.value_of("folder").unwrap_or("");
    let card_file = clp.value_of("cards").unwrap_or("cards.crd");
    let temp_file = clp.value_of("temp").unwrap_or("main.ito");

    let s = include_str!("text/basic.ito");
    let s2 = s.replace("<card_file>", card_file);

    std::fs::create_dir_all(folder)?;
    let card_path = std::path::PathBuf::from(folder).join(card_file);
    if !std::fs::metadata(&card_path).is_ok() {
        std::fs::write(&card_path, include_str!("text/dummy_cards.crd"))?;
    }
    let temp_path = std::path::PathBuf::from(folder).join(temp_file);
    std::fs::write(&temp_path, s2)?;

    Ok(())
}

pub fn build_table(clp:&ArgMatches) -> anyhow::Result<()>{
    let f = clp.value_of("card_file").e_str("No Card File Provided")?;
    let cf = std::fs::read_to_string(f)?;
    let cl = card_format::parse_cards(&cf)?;

    for (n,c) in cl.iter().enumerate() {
        println!("{:2} : {}",n,c.name);
    }


    


    Ok(())
}
