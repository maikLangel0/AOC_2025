use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("src\\day_3\\input.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut buf: [u8; _] = [0; 100];
    let mut res_first: u16 = 0;
    let mut res_second: u64 = 0;

    for line in reader.lines() {

        let line = line.expect("tf line-read failed!");

        buf.copy_from_slice(line.as_bytes());
        buf = buf.map(|n| n - '0' as u8); // Converting from ascii representation of number to u8

        solve_first(&buf, &mut res_first);
        solve_second(&buf, 12, &mut res_second);
    }

    println!("RES 1: {}", res_first);
    println!("RES 2: {}", res_second);
}

fn solve_first(buf: &[u8; 100], res: &mut u16) {
    //              pos|val
    let mut first: (u8, u8) = (0, 1);
    let mut second: u8 = 1;

    let mut idx: u8 = 0;

    for num in buf.iter().take( buf.len() - 1 ) {
        idx += 1;

        if *num == 9 {
            first = (idx, 9);
            break;
        }

        if *num > first.1 {
            first = (idx, *num);
        }
    }

    for num in buf.iter().skip(first.0 as usize) {
        if *num == 9 {
            second = 9;
            break;
        }

        if *num > second {
            second = *num;
        }
    }

    *res += ( (first.1 * 10) + second ) as u16;
}

fn solve_second(buf: &[u8; 100], jolt_amount: usize, res: &mut u64) {
    debug_assert!(jolt_amount <= buf.len(), "Jolt amount cannot be bigger than buf len.");

    let mut idx_of_curr_jolt: u8 = 0;

    for jolt in (1..=jolt_amount).rev() {
        let buf_iter = buf.iter()
            .take(buf.len() - jolt as usize + 1) // No need to iter over jolts that go past the current max digit
            .skip(idx_of_curr_jolt as usize); // Skips to the idx where last jolt was found

        //             pos|val
        let mut curr: (u8, u8) = (0, 1);

        let mut idx: u8 = 0;
        for num in buf_iter {
            idx += 1;

            if *num == 9 {
                curr = (idx, 9);
                break
            }

            if curr.1 < *num {
                curr.1 = *num;
                curr.0 = idx;
            }
        }

        idx_of_curr_jolt += curr.0;
        *res += curr.1 as u64 * (10 as u64).pow( (jolt - 1) as u32 );
    }
}
