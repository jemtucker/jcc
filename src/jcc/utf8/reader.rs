use std::io::{Bytes, Read};
use thiserror;

pub enum Utf8Reader<R> {
    Active(Bytes<R>),
    Fused,
}

#[derive(thiserror::Error, Debug)]
pub enum Utf8ReadError {
    #[error("{0:?}")]
    IoErr(#[from] std::io::Error),
    #[error("Invalid UTF8 sequence")]
    DecodeErr(#[from] std::str::Utf8Error),
}

impl<R: Read> Utf8Reader<R> {
    pub fn new(source: R) -> Self {
        Utf8Reader::Active(source.bytes())
    }
}

impl<R: Read> Iterator for Utf8Reader<R> {
    type Item = Result<char, Utf8ReadError>;
    fn next(&mut self) -> Option<Self::Item> {
        use Utf8Reader::*;

        let Active(source) = self else {
            return None;
        };

        let mut bytes = [0u8; 4];

        for n in 0..4 {
            bytes[n] = match source.next() {
                None => {
                    // EOF
                    *self = Fused;
                    return match (n, std::str::from_utf8(&bytes[..n])) {
                        // EOF at char boundary
                        (0, _) => None,

                        // Incomplete char at EOF
                        (_, Err(e)) => Some(Err(e.into())),

                        // Returned in previous loop iteration
                        _ => unreachable!(),
                    };
                }
                Some(Err(e)) => {
                    // I/O Error in reader
                    *self = Fused;
                    return Some(Err(e.into()));
                }

                // Byte available
                Some(Ok(b)) => b,
            };

            match std::str::from_utf8(&bytes[..=n]) {
                // Complete char has been read
                Ok(s) => {
                    return Some(Ok(s.chars().next().unwrap()));
                }

                // Invalid UTF-8 sequence in input
                Err(e) if e.error_len().is_some() => {
                    *self = Fused;
                    return Some(Err(e.into()));
                }
                _ => (),
            };
        }

        // 4 bytes is the maximum length of a UTF-8 sequence
        unreachable!()
    }
}
