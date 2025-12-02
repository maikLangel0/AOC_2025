use std::{error::Error, fs::File, io::{BufRead, BufReader}, result::Result};


// Illegal ids:
// starer med 0
// ingen sekvens av tall etter hverandre

fn main() -> Result<(), Box<dyn Error>> {
    let mut result1: u64 = 0;
    let mut result2: u64 = 0;

    let mut buf: Vec<u8> = Vec::new();
    let file = File::open("src\\day_2\\input.txt")?;

    let mut reader = BufReader::new(&file);

    loop {
        match reader.read_until(0x2C, &mut buf) {
            Ok(res) => { if res == 0 { break } },
            Err(_) => break
        };

        let id_range_string = String::from_utf8(buf.clone())?;
        buf.clear();

        let (bottom, top) = id_range_string.split_once('-').map(|e| (e.0.trim(), e.1.trim().replace(',', ""))).unwrap();
        let bottom = bottom.parse::<u64>()?;
        let top = top.parse::<u64>()?;

        solve_first(bottom, top, &mut result1);
        solve_second(bottom, top, &mut result2);
    }

    println!("\n{}", result1);
    println!("{}", result2);
    Ok(())
}

fn split_half(n: u64) -> (u64, u64) {
    let digits: u32 = n.ilog10() + 1;
    let factor: u64 = (10 as u64).pow(digits / 2);

    (n / factor, n % factor)
}

fn solve_first(bottom: u64, top: u64, res: &mut u64) {
    if (top.ilog10() + 1) % 2 != 0 && (bottom.ilog10() + 1) % 2 != 0 {
        return;
    }

    for id in bottom..=top {
        let digits = id.ilog10() + 1;

        if digits % 2 != 0 {
            continue;
        }

        let (num, ber) = split_half(id);

        if num == ber { *res += id }
    }
}

fn to_digits(mut v: u64) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::with_capacity(16);

    while v > 0 {
        let n = (v % 10) as u8;
        v /= 10;
        digits.push(n);
    }
    digits
}

fn solve_second(bottom: u64, top: u64, res: &mut u64) {
    for id in bottom..=top {
        let idv = to_digits(id);
        let len = idv.len();

        let mut reps = false;

        for step in 1..=len / 2 {
            if len % step != 0 { continue }

            let pattern = &idv[0..step];
            let mut all_match = true;

            for chunk in idv.chunks(step).skip(1) {
                if chunk != pattern {
                    all_match = false;
                    break;
                }
            }
            if all_match { reps = true; break }
        }
        if reps { *res += id }
    }
}
