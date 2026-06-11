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
                let ix = (x - 64) as i16 as f16 / 64.0;
                let iy = (y - 48) as i16 as f16 / 48.0;
                let c = (3.9 - (ix * ix + iy * iy) * 2.0) as u16 * 85;
                context!(2, y);
                context!(3, c);
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
    let code = "
    fn add(x: f16, y: f16) -> f16 {
        x + y
    }
    fn main() {
        dbg!(add(6.7, 42.0));
    }
    ";
    let code = "
    fn inc_ptr(ptr: &u16) {
        *ptr = *ptr + 1;
    }
    fn main() {
        let x = 67;
        inc_ptr(&x);
        inc_ptr(&x);
        dbg!(x);
    }
    ";
    let code = "
    fn fib(n: u16) -> u16 {
        let result = n;
        let k = n in for k downto 0 {
            for k downto 0 {
                result = fib(n - 2) + fib(n - 1);
                k = 0;
            };
            k = 0;
        };
        result
    }

    fn main() {
        dbg!(1, fib(1));
        dbg!(2, fib(2));
        dbg!(3, fib(3));
        dbg!(4, fib(4));
        dbg!(5, fib(5));
        let x = 10 in for x downto 0 {
            let y = 10 - x;
            dbg!(y, fib(y));
        };
    }
    ";
    let ir = ir_generator::compile(code).unwrap();
    dbg!(&ir);
    let asm = chasm_amine_backend::compile_chasm_to_amine(&ir);
    println!("{}", asm.iter().map(|s| s.to_string()).collect::<String>());
}
