use templito::func_man::{BasicFuncs, FuncManager, WithFuncs};

fn main() {
    let fman = BasicFuncs::new()
        .with_defaults()
        .with_exec()
        .with_free_files();

    fman.print_filter("d");
}
