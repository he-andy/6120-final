use bril_rs::load_program;
use preprocessing::{preprocess, utils::code_to_bb};

fn main() {
    let mut prog = load_program();
    for func in prog.functions.iter_mut() {
        function_preprocess(func);
    }
    println!("{}", prog);
}

fn function_preprocess(func: &mut bril_rs::Function) {
    let bbs = code_to_bb(func.instrs.clone());
    let fnname = func.name.clone();
    let code = bbs
        .iter()
        .enumerate()
        .map(|(i, bb)| preprocess::remove_data_dependencies(&bb, &fnname, i))
        .collect::<Vec<_>>();
    func.instrs = code.into_iter().flatten().collect();
}


