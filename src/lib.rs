pub mod error;
pub use error::HeaviError;

use regex::bytes::Regex;
use std::io::{Read, Write};

const BUF_SIZE: usize = 4096;

pub fn heavi_line<R: Read, W: Write>(
    mut input: R,
    mut output: W,
    pattern: &str,
) -> Result<(), HeaviError> {
    let re = Regex::new(&format!("(?m){}", pattern))?;

    let mut dbuf = [0; BUF_SIZE];
    let mut n = input.read(&mut dbuf)?;
    let mut dbuf_len = n;
    // find the match
    while dbuf_len > 0 {
        let idx = match re.find(&dbuf) {
            Some(m) => Some(m.end()),
            None => None,
        };
        if let Some(idx) = idx {
            // shift out the match
            // |_match_|__dbuf_len-idx__|
            //      idx^        dbuf_len^
            dbuf.copy_within(idx.., 0);
            n = input.read(&mut dbuf[dbuf_len - idx..])?;

            // |__dbuf_len-idx__|_n_|
            dbuf_len = dbuf_len - idx + n;
            break;
        }
        // iterate through the file
        if dbuf_len == BUF_SIZE {
            // |__buf1__|__buf2__|
            dbuf.copy_within(BUF_SIZE / 2.., 0);
            n = input.read(&mut dbuf[BUF_SIZE / 2..])?;

            // |__buf2__|__n__|
            dbuf_len = BUF_SIZE / 2 + n;
        } else {
            // no more data to read in
            dbuf = [0; BUF_SIZE];
            dbuf_len = 0;
        }
    }

    // find the newline
    while dbuf_len > 0 {
        let idx = dbuf.iter().position(|&r| r == 10);
        if let Some(idx) = idx {
            let idx = idx + 1;
            // shift out the newline
            // |_CR_|__dbuf_len-idx__|
            //   idx^        dbuf_len^
            dbuf.copy_within(idx.., 0);
            n = input.read(&mut dbuf[BUF_SIZE - idx..])?;

            // |__dbuf_len-idx__|_n_|
            dbuf_len = dbuf_len - idx + n;
            break;
        }
        n = input.read(&mut dbuf)?;
        dbuf_len = n;
    }

    // print the rest of the file
    while dbuf_len > 0 {
        output.write(&dbuf[..dbuf_len])?;
        dbuf_len = input.read(&mut dbuf)?;
    }
    Ok(())
}

pub fn heavi_line_inv<R: Read, W: Write>(
    mut input: R,
    mut output: W,
    pattern: &str,
) -> Result<(), HeaviError> {
    let re = Regex::new(&format!("(?m){}", pattern))?;

    let mut dbuf = [0; BUF_SIZE];
    let mut n = input.read(&mut dbuf)?;
    let mut dbuf_len = n;
    while dbuf_len > 0 {
        if let Some(m) = re.find(&dbuf[..dbuf_len]) {
            let sect = &dbuf[..m.start()];
            // last newline before pattern
            if let Some(idx) = sect.iter().rev().position(|&r| r == 10) {
                output.write(&dbuf[..m.start() - idx])?;
            }
            break;
        }
        if dbuf_len == BUF_SIZE {
            output.write(&dbuf[..BUF_SIZE / 2])?;

            // iterate through the file
            // |__buf1__|__buf2__|
            dbuf.copy_within(BUF_SIZE / 2.., 0);
            n = input.read(&mut dbuf[BUF_SIZE / 2..])?;

            // |__buf2__|__n__|
            dbuf_len = BUF_SIZE / 2 + n;
        } else {
            // no more data to read in
            output.write(&dbuf[..dbuf_len])?;
            break;
        }
    }
    Ok(())
}

pub fn heavi<R: Read, W: Write>(
    mut input: R,
    mut output: W,
    pattern: &str,
) -> Result<(), HeaviError> {
    let re = Regex::new(&format!("(?m){}", pattern))?;

    let mut dbuf = [0; BUF_SIZE];
    let mut n = input.read(&mut dbuf)?;
    let mut dbuf_len = n;
    // find the match
    while dbuf_len > 0 {
        let idx = match re.find(&dbuf) {
            Some(m) => Some(m.end()),
            None => None,
        };
        if let Some(idx) = idx {
            // shift out the match
            // |_match_|__dbuf_len-idx__|
            //      idx^        dbuf_len^
            dbuf.copy_within(idx.., 0);
            n = input.read(&mut dbuf[dbuf_len - idx..])?;

            // |__dbuf_len-idx__|_n_|
            dbuf_len = dbuf_len - idx + n;
            break;
        }
        // iterate through the file
        if dbuf_len == BUF_SIZE {
            // |__buf1__|__buf2__|
            dbuf.copy_within(BUF_SIZE / 2.., 0);
            n = input.read(&mut dbuf[BUF_SIZE / 2..])?;

            // |__buf2__|__n__|
            dbuf_len = BUF_SIZE / 2 + n;
        } else {
            // no more data to read in
            dbuf = [0; BUF_SIZE];
            dbuf_len = 0;
        }
    }

    // print the rest of the file
    while dbuf_len > 0 {
        output.write(&dbuf[..dbuf_len])?;
        dbuf_len = input.read(&mut dbuf)?;
    }
    Ok(())
}

pub fn heavi_inv<R: Read, W: Write>(
    mut input: R,
    mut output: W,
    pattern: &str,
) -> Result<(), HeaviError> {
    let re = Regex::new(&format!("(?m){}", pattern))?;

    let mut dbuf = [0; BUF_SIZE];
    let mut n = input.read(&mut dbuf)?;
    let mut dbuf_len = n;
    while dbuf_len > 0 {
        if let Some(m) = re.find(&dbuf[..dbuf_len]) {
            output.write(&dbuf[..m.start()])?;
            break;
        }
        if dbuf_len == BUF_SIZE {
            output.write(&dbuf[..BUF_SIZE / 2])?;

            // iterate through the file
            // |__buf1__|__buf2__|
            dbuf.copy_within(BUF_SIZE / 2.., 0);
            n = input.read(&mut dbuf[BUF_SIZE / 2..])?;

            // |__buf2__|__n__|
            dbuf_len = BUF_SIZE / 2 + n;
        } else {
            // no more data to read in
            output.write(&dbuf[..dbuf_len])?;
            break;
        }
    }
    Ok(())
}
