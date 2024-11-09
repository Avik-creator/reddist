#[derive(Clone, Debug)]
pub enum RespType{
    SimpleString(String),
    BulkString(String),
    SimpleError(String),
}

use bytes::{Bytes, BytesMut};
use super::RespError;

impl RespType {
    pub fn parse(buffer: BytesMut) -> Result<(RespType, usize), RespError> {
        let c = buffer[0] as char;
        return match c {
            '$' => Self::parse_bulk_string(buffer),
            '+' => Self::parse_simple_string(buffer),
            _ => Err(RespError::Other(String::from("Invalid RESP data type"))),
        };
    }

    pub fn parse_bulk_string(buffer: BytesMut) -> Result<(RespType, usize), RespError> {
        let (bulkstr_len, bytes_consumed) = if let Some((buf_data, len)) = Self::read_till_crlf(&buffer[1..]) {
            let bulkstr_len = Self::parse_usize_from_buf(buf_data)?;
            (bulkstr_len, len + 1)
        } else {
            return Err(RespError::InvalidBulkString(String::from(
                "Invalid bulk string length",
            )));
        };


        let bulkstr_end_idx = bytes_consumed + bulkstr_len as usize;
        if bulkstr_end_idx >= buffer.len(){
            return Err(RespError::InvalidBulkString(String::from(
                "Invalid bulk string length",
            )));
        }

        let bulkStr = String::from_utf8(buffer[bytes_consumed..bulkstr_end_idx].to_vec());
        match bulkStr {
            Ok(s) => Ok((RespType::BulkString(s), bulkstr_end_idx + 2)),
            Err(_) => Err(RespError::InvalidBulkString(String::from(
                "Invalid bulk string",
            ))),
        }
    }

    pub fn to_bytes(&self) -> Bytes{
        return match self {
            RespType::SimpleString(ss) => Bytes::from_iter(format!("+{}\r\n", ss).into_bytes()),
            RespType::BulkString(bs) => {
                let bulkstr_bytes = format!("${}\r\n{}\r\n", bs.chars().count(), bs).into_bytes();
                Bytes::from_iter(bulkstr_bytes)
            }
            RespType::SimpleError(es) => Bytes::from_iter(format!("-{}\r\n", es).into_bytes()),
        };
    }

    fn read_till_crlf(buf: &[u8]) -> Option<(&[u8], usize)> {
        for i in 1..buf.len() {
            if buf[i - 1] == b'\r' && buf[i] == b'\n' {
                return Some((&buf[0..(i - 1)], i + 1));
            }
        }

        None
    }

    fn parse_usize_from_buf(buf: &[u8]) -> Result<usize, RespError> {
        let utf8_str = String::from_utf8(buf.to_vec());
        let parsed_int = match utf8_str {
            Ok(s) => {
                let int = s.parse::<usize>();
                match int {
                    Ok(n) => Ok(n),
                    Err(_) => Err(RespError::InvalidBulkString(String::from(
                        "Invalid bulk string length",
                    ))),
                }
            }
            Err(_) => Err(RespError::InvalidBulkString(String::from(
                "Invalid bulk string length",
            ))),
        };
        parsed_int
    }

     pub fn parse_simple_string(buffer: BytesMut) -> Result<(RespType, usize), RespError> {
        // read until CRLF and parse the bytes into an UTF-8 string.
        if let Some((buf_data, len)) = Self::read_till_crlf(&buffer[1..]) {
            let utf8_str = String::from_utf8(buf_data.to_vec());

            return match utf8_str {
                Ok(simple_str) => Ok((RespType::SimpleString(simple_str), len + 1)),
                Err(_) => {
                    return Err(RespError::InvalidSimpleString(String::from(
                        "Simple string value is not a valid UTF-8 string",
                    )))
                }
            };
        }

        Err(RespError::InvalidSimpleString(String::from(
            "Invalid value for simple string",
        )))
    }



    



}