use std::str::FromStr;
use templito::{
    func_man::BasicFuncs, temp_man::BasicTemps as TMan, TData, TreeTemplate, WithFuncs,
};

pub fn import_templates_tdata(
    d: &TData,
    tman: &mut TMan,
    fman: &mut BasicFuncs,
) -> anyhow::Result<()> {
    match d {
        TData::String(s) => import_templates(s, tman, fman),
        TData::List(l) => {
            for s in l {
                import_templates(&s.to_string(), tman, fman)?;
            }
            Ok(())
        }
        _ => {
            return err_tools::e_str("Can only import string and lists of templates");
        }
    }
}

/// Adds the global templates to the TMan
pub fn import_templates_list<S: AsRef<str>>(
    l: &[S],
    tman: &mut TMan,
    fman: &BasicFuncs,
) -> anyhow::Result<()> {
    for s in l {
        import_templates(s.as_ref(), tman, fman)?;
    }
    Ok(())
}

pub fn import_templates(s: &str, tman: &mut TMan, fman: &BasicFuncs) -> anyhow::Result<()> {
    let t = std::fs::read_to_string(s)?;
    let tp = TreeTemplate::from_str(&t)?;
    Ok(())
}
