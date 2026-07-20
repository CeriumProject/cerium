use clipboard::ClipboardProvider;
use ir_generator::error::FormatError;

fn main() {
    let code = include_str!("../examples/mem_generic.cer");
    let ir = match ir_generator::compile(code) {
        Ok(ir) => ir,
        Err(err) => {
            println!("{}", err.format(code));
            return;
        }
    };
    println!(
        "{}",
        ir.iter()
            .map(|section| section.to_string())
            .collect::<String>()
    );
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    let asm_str = asm.iter().map(|s| s.to_string()).collect::<String>();
    println!("{}", &asm_str);
    clipboard::ClipboardContext::new().unwrap().set_contents(asm_str).unwrap();
}
