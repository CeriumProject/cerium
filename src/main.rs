fn main() {
    let code = "
const HEAP: &&u16 = &(0 alias &u16);

fn malloc(size: u16) -> &u16 {
    let result = *HEAP;
    result = result - size;
    *HEAP = result;
    result
}

fn free(ptr: &u16) {
    let leckei = ptr;
}

fn main() {
    let arr = malloc(1);
    *arr = 67;
    dbg!(*arr);
}
";
    let ir = ir_generator::compile(code).unwrap();
    dbg!(&ir);
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
