use ir_generator::error::FormatError;

fn main() {
    let code = "
fn idk(x: i16) -> i16 {
    x + 4.3
}
fn main() {
}
";
    // let code = include_str!("../examples/mode7.cer");
    let ir = match ir_generator::compile(code) {
        Ok(ir) => ir,
        Err(err) => {
            println!("{}", err.format(code));
            return;
        }
    };
    //dbg!(&ir);
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
