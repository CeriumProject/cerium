use chasm_ir::Section;
use ir_generator::error::FormatError;

fn main() {
    let code = include_str!("../examples/rasterize.cer");
    let ir = match ir_generator::compile(code) {
        Ok(ir) => ir,
        Err(err) => {
            println!("{}", err.format(code));
            return;
        }
    };
    println!("{}", ir.iter().map(|section| section.to_string()).collect::<String>());
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
