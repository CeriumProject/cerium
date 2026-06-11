fn main() {
    let code = "
fn swap(x: &u16, y: &u16) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn main() {
    let x = 6;
    let y = 7;
    swap(&x, &y);
    dbg!(x, y);
}
";
    let ir = ir_generator::compile(code).unwrap();
    dbg!(&ir);
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
