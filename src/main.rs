fn main() {
    let code = "
fn color_at(x: u16, y: u16) -> u16 {
    x * x + y * y
}

fn draw(getter: &fn(u16,u16) -> u16) {
    device!(0);
    context!(0, 1);
    let x = 128 in for x downto 0 {
        context!(1, x);
        let y = 96 in for y downto 0 {
            context!(2, y);
            context!(3, getter(x, y));
            send!();
        };
    };
}

fn flip() {
    device!(0);
    context!(0, 2);
    send!();
}

fn main() {
    let f = color_at;
    loop {
        draw(f);
        flip();
    }
}
";
    let ir = ir_generator::compile(code).unwrap();
    dbg!(&ir);
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
