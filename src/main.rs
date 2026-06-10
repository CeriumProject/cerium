fn main() {
    let code = "fn fib(n: u16) -> u16 {
        let x = 0;
        let y = 1;
        for n downto 0 {
            let z = x + y;
            x = y;
            y = z;
        }
        x
    }";
    let code = "fn main() { let x = 10; x = 20; }";
    let code = "
    fn sqrt(radicand: f16) -> f16 {
        let approx = (radicand alias u16 / 2 + 7680) alias f16 in
            (radicand / approx + approx) * 0.5
    }
    ";
    let code = "
    fn mem_copy(dst: &u16, src: &u16, len: u16) {
        for len downto 0 {
            *dst = *src;
            dst = dst + 1;
            src = src + 1;
        }
    }
    ";
    let code = "
    fn main() {
        let x = 100 in for x downto 0 {
            dbg!(x);
            x = 3;
        }
    }
    ";
    let code = "
    fn draw() {
        device!(0);
        context!(0, 1);
        let x = 128 in for x downto 0 {
            context!(1, x);
            let y = 96 in for y downto 0 {
                context!(2, y);
                context!(3, x + y);
                send!();
            }
        }
    }

    fn flip() {
        device!(0);
        context!(0, 2);
        send!();
    }

    fn main() {
        loop {
            draw();
            flip();
        }
    }
    ";
    let ir = ir_generator::compile(code).unwrap();
    dbg!(&ir);
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
