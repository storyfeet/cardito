use err_tools::*;
use std::path::PathBuf;
use std::str::FromStr;
use templito::{func_man::BasicFuncs, temp_man::BasicTemps as TMan, TData, TreeTemplate};

pub fn import_templates_tdata(d: &TData, tman: &mut TMan, fman: &BasicFuncs) -> anyhow::Result<()> {
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

pub fn import_templates(s: &str, tman: &mut TMan, fman: &BasicFuncs) -> anyhow::Result<()> {
    let w = Walker::new(PathBuf::from(s));
    for f in w {
        let s = std::fs::read_to_string(&f).e_string(format!("could not read file {:?}", f))?;
        let tt = TreeTemplate::from_str(&s)?;
        tt.run(&[], tman, fman)?;
    }
    Ok(())
}

pub struct Walker {
    stack: Vec<PathBuf>,
}

impl Walker {
    pub fn new(p: PathBuf) -> Self {
        Walker { stack: vec![p] }
    }
}

impl Iterator for Walker {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.stack.pop()?;
            let meta = match std::fs::metadata(&p) {
                Ok(meta) => meta,
                Err(_) => continue,
            };
            if meta.is_file() {
                return Some(p);
            }
            if meta.is_dir() {
                let dir = match std::fs::read_dir(&p) {
                    Ok(dir) => dir,
                    Err(_) => continue,
                };
                for x in dir {
                    if let Ok(f) = x {
                        self.stack.push(f.path());
                    }
                }
            }
        }
    }
}
