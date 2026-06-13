fn main() {
    let code = "
const mem::HEAD: &&u16 = &(0 alias &u16);

fn mem::alloc(size: u16) -> &u16 {
    let result = *mem::HEAD;
    result = result - size;
    *mem::HEAD = result;
    result
}

fn main() {
    let arr = mem::alloc(42);
    arr[42] = 67;
    dbg!(arr[42]);
}
";
    let ir = ir_generator::compile(code).unwrap();
    dbg!(&ir);
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
