fn main() {
    let code = "
const NUMBERS: &&u16 = &[&[1, 2, 3], &[4, 5]];

fn main() {
    dbg!(NUMBERS[0][0]);
    dbg!(NUMBERS[0][1]);
    dbg!(NUMBERS[0][2]);
    dbg!(NUMBERS[1][0]);
    dbg!(NUMBERS[1][1]);
}
";
    let ir = ir_generator::compile(code).unwrap();
    dbg!(&ir);
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
