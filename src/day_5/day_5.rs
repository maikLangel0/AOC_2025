use std::{fs::File, io::{BufRead, BufReader, Read}};

const BUFSIZE: usize = 15;

/// `buf` needs to be least significant digit to most significant
#[inline(always)]
fn buf_to_num(buf: &[u8; BUFSIZE]) -> u64 {
    let mut res: u64 = 0;

    for (i, num) in buf.iter().enumerate().rev() {
        res += *num as u64 * 10u64.pow( (i) as u32 );
    }
    res
}

/// Does basically the exact same as the .fill() on [[T]], but here it does only exactly what it has to
#[inline(always)]
fn fill_to_index(buf: &mut [u8; BUFSIZE], idx: usize, val: u8) {
    for num in buf[0..idx].iter_mut() {
        *num = val.clone();
    }
}

// I COULD OVERKILL THE SOLUTION TO PART 2 IF I COULD USE MY BRAIN BETTER; I FEEL LIKE IT
// could possibly do some in-place sorting on insertion instead of allocating a new Vec
fn parse_ranges<T: Read>(reader: &mut BufReader<T>) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = Vec::with_capacity(256);

    let mut topbuf: [u8; BUFSIZE] = [0; _];
    let mut botbuf: [u8; BUFSIZE] = [0; _];

    for line in reader.lines() {
        let line = line.expect("Line read didnt work fuck you");

        if line.is_empty() { break } // End of ranges

        let mut top: u64 = 0; // Niche asf optimization cuz I can turn topbuf to num when c == '-'

        let mut write_to_top = true;
        let mut digit: usize = 0; // current digit placement in buf

        for c in line.bytes().rev() {
            match c {
                b'-' => { // Hits this only once so fill_to_index & fetching top is optimal here
                    write_to_top = !write_to_top;

                    top = buf_to_num(&topbuf);
                    fill_to_index(&mut topbuf, digit, 0);

                    digit = 0;
                },
                _   => {
                    let c = c - b'0';

                    if write_to_top {
                        topbuf[digit] = c;
                    } else {
                        botbuf[digit] = c;
                    }
                    digit += 1;
                }
            }
        }
        // println!("bot: {:?} | top: {:?}", botbuf, topbuf);

        ranges.push( (buf_to_num(&botbuf), top) );

        fill_to_index(&mut botbuf, digit, 0);
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged: Vec<(u64, u64)> = Vec::with_capacity( ranges.len() / 2 + 1 );
    let mut curr = ranges[0];

    for (bottom, top) in &ranges[1..] {
        if *bottom <= curr.1 + 1 {     // If iters bottom is less then or equal to curr + 1 (Ex: (3,5) (6,7) should become (3,7) )
            curr.1 = curr.1.max(*top); // Sets curr top to either itself or new top
        } else {                       // if not, curr is a completed range
            merged.push(curr);
            curr = (*bottom, *top);    // Set curr to iters' values
        }
    }

    merged.push(curr);
    merged
}

fn parse_ids<T: Read>(reader: &mut BufReader<T>) -> Vec<u64> {
    let mut ids: Vec<u64> = Vec::with_capacity(1028);
    let mut idbuf: [u8; BUFSIZE] = [0; BUFSIZE];

    for line in reader.lines() {
        let line = line.expect("Line read didnt work fuck you");

        if line.is_empty() { break } // End of ranges

        let mut digit: usize = 0; // current digit placement in buf

        for c in line.bytes().rev() {
            let c = c - b'0';
            idbuf[digit] = c;
            digit += 1;
        }
        ids.push( buf_to_num(&idbuf) );
        idbuf.fill(0);
    }
    ids
}

fn main() {
    let file: File = File::open("src\\day_5\\input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let ranges: Vec<(u64, u64)> = parse_ranges(&mut reader);
    let ids:    Vec<u64>        = parse_ids(&mut reader);

    // Solves 1
    let mut in_range_amount: u16 = 0;
    for id in ids {
        for range in &ranges {
            if id >= range.0 && id <= range.1 {
                in_range_amount += 1;
                break;
            }
        }
    }

    // Solves 2
    let mut res2 = 0;
    for range in &ranges {
        res2 += range.1 - range.0 + 1; // Sub bottom from top + 1 to get all nums in inclusive range
    }

    println!("RES 1: {}", in_range_amount);
    println!("RES 2: {}", res2);
}
