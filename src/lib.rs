pub mod error;
pub use error::HeaviError;

use regex::bytes::Regex;
use std::io::{BufRead, Write};

const BUF_SIZE: usize = 4096;

#[derive(PartialEq, Debug)]
pub enum HeaviInst {
    Cont, // continue processing
    Stop, // stop processing
}

pub trait HeaviParser {
    fn before_match(&mut self, buf: &[u8]) -> Result<HeaviInst, HeaviError>;
    fn at_match(&mut self, buf: &[u8]) -> Result<HeaviInst, HeaviError>;
    fn after_match(&mut self, buf: &[u8]) -> Result<HeaviInst, HeaviError>;

    fn parse<R: BufRead>(&mut self, input: R, pattern: &str) -> Result<(), HeaviError>;

    fn parse_bytes<R: BufRead>(&mut self, mut input: R, pattern: &str) -> Result<(), HeaviError> {
        let re = Regex::new(&format!("(?m){}", pattern))?;
        let mut dbuf = [0; BUF_SIZE];
        let mut dbuf_len = read(&mut input, &mut dbuf)?;

        while dbuf_len > 0 {
            if let Some(m) = re.find(&dbuf) {
                if self.before_match(&dbuf[..m.start()])? == HeaviInst::Stop {
                    return Ok(());
                }
                if self.at_match(&dbuf[m.start()..m.end()])? == HeaviInst::Stop {
                    return Ok(());
                }
                if self.after_match(&dbuf[m.end()..dbuf_len])? == HeaviInst::Stop {
                    return Ok(());
                }
                dbuf_len = read(&mut input, &mut dbuf)?;
                break;
            }
            if dbuf_len == BUF_SIZE {
                // discard first half of dbuf
                if self.before_match(&dbuf[..BUF_SIZE / 2])? == HeaviInst::Stop {
                    return Ok(());
                }
                dbuf.copy_within(BUF_SIZE / 2.., 0);
                // read in next half of dbuf
                let n = read(&mut input, &mut dbuf[BUF_SIZE / 2..])?;
                dbuf_len = BUF_SIZE / 2 + n;
            } else {
                // no more data to read, we never found a match
                self.before_match(&dbuf[..dbuf_len])?;
                return Ok(());
            }
        }

        // after match, go through the rest of the data
        while dbuf_len > 0 {
            if self.after_match(&dbuf[..dbuf_len])? == HeaviInst::Stop {
                return Ok(());
            }
            dbuf_len = read(&mut input, &mut dbuf)?;
        }

        Ok(())
    }
    fn parse_lines<R: BufRead>(&mut self, mut input: R, pattern: &str) -> Result<(), HeaviError> {
        // NOTE: patterns may not contain newlines in this mode
        let re = Regex::new(pattern)?;

        let mut last_line = vec![];
        input.read_until(b'\n', &mut last_line)?;
        if re.find(&last_line[..last_line.len() - 1]).is_some() {
            if self.at_match(&last_line)? == HeaviInst::Stop {
                return Ok(());
            }
        } else {
            loop {
                let mut line = vec![];
                if self.before_match(&last_line)? == HeaviInst::Stop {
                    return Ok(());
                }
                if input.read_until(b'\n', &mut line)? == 0 {
                    break;
                }
                // remove newline when checking for a match
                if re.find(&line[..line.len() - 1]).is_some() {
                    if self.at_match(&line)? == HeaviInst::Stop {
                        return Ok(());
                    }
                    break;
                }
                last_line = line;
            }
        }

        // after match
        let mut line = vec![];
        while input.read_until(b'\n', &mut line)? != 0 {
            if self.after_match(&line)? == HeaviInst::Stop {
                return Ok(());
            }
            line.clear();
        }
        Ok(())
    }
}

// try to fill the buffer and returns how many bytes read
fn read<R: BufRead>(input: &mut R, buf: &mut [u8]) -> Result<usize, HeaviError> {
    let mut total_read = input.read(buf)?;
    while total_read < buf.len() {
        let n = input.read(&mut buf[total_read..])?;
        total_read += n;
        if n == 0 {
            break;
        }
    }
    Ok(total_read)
}

pub struct Heavi<W: Write> {
    pub line_mode: bool,
    pub invert: bool,
    pub inclusive: bool,
    pub output: W,
}

impl<W: Write> HeaviParser for Heavi<W> {
    fn before_match(&mut self, buf: &[u8]) -> Result<HeaviInst, HeaviError> {
        if self.invert {
            if self.output.write(buf).is_err() {
                return Ok(HeaviInst::Stop);
            }
        }
        Ok(HeaviInst::Cont)
    }
    fn at_match(&mut self, buf: &[u8]) -> Result<HeaviInst, HeaviError> {
        // only print if inclusive
        if self.inclusive {
            if self.output.write(buf).is_err() {
                return Ok(HeaviInst::Stop);
            }
        }
        Ok(if self.invert {
            HeaviInst::Stop
        } else {
            HeaviInst::Cont
        })
    }
    fn after_match(&mut self, buf: &[u8]) -> Result<HeaviInst, HeaviError> {
        // at_match will quit if the output is inverted
        if self.output.write(buf).is_err() {
            return Ok(HeaviInst::Stop);
        }
        Ok(HeaviInst::Cont)
    }
    fn parse<R: BufRead>(&mut self, input: R, pattern: &str) -> Result<(), HeaviError> {
        if self.line_mode {
            self.parse_lines(input, pattern)
        } else {
            self.parse_bytes(input, pattern)
        }
    }
}
