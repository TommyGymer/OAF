use std::string::FromUtf8Error;

#[derive(Debug, Clone)]
pub enum BufferError {
    BufferEmpty,
    InvalidUtf8String(FromUtf8Error),
    UsizeTooBig,
}

fn usize_as_u16_as_bytes(len: usize) -> Result<Vec<u8>, BufferError> {
    if len > u16::MAX as usize {
        Err(BufferError::UsizeTooBig)
    } else {
        Ok((len as u16).to_le_bytes().to_vec())
    }
}

#[derive(Debug)]
pub struct Buffer {
    v: Vec<u8>,
}

impl PartialEq<Self> for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl PartialEq<Vec<u8>> for Buffer {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.v == *other
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            v: vec!(),
        }
    }

    pub fn from(v: Vec<u8>) -> Buffer{
        Buffer {
            v,
        }
    }

    pub fn append_usize(&mut self, len: usize) -> Result<(), BufferError> {
        self.v.append(&mut usize_as_u16_as_bytes(len)?);
        Ok(())
    }

    pub fn append_string(&mut self, str: &String) -> Result<(), BufferError> {
        let b = str.as_bytes();
        self.append_usize(b.len())?;
        self.v.extend_from_slice(b);
        Ok(())
    }

    pub fn append(&mut self, b: &mut Buffer) {
        self.v.append(&mut b.v);
    }

    pub fn append_u32(&mut self, n: u32) {
        self.v.extend_from_slice(&n.to_le_bytes());
    }

    pub fn append_f32(&mut self, n: f32) {
        self.v.extend_from_slice(&n.to_le_bytes());
    }

    pub fn append_u8(&mut self, n: u8) {
        self.v.extend_from_slice(&n.to_le_bytes());
    }

    pub fn pop_n_bytes(&mut self, n: usize) -> Result<Vec<u8>, BufferError> {
        let mut res = vec![];

        for i in 0..n {
            let ob = self.v.get(i);
            match ob {
                None => return Err(BufferError::BufferEmpty),
                Some(b) => res.push(*b),
            }
        }
        self.v = self.v.split_at(res.len()).1.to_vec();
        Ok(res)
    }

    pub fn pop_usize(&mut self) -> Result<usize, BufferError> {
        let bytes = self.pop_n_bytes(2)?;
        Ok(u16::from_le_bytes(bytes.try_into().unwrap()) as usize)
    }

    pub fn pop_string(&mut self) -> Result<String, BufferError> {
        let len = self.pop_usize()?;
        match String::from_utf8(self.pop_n_bytes(len)?) {
            Ok(str) => Ok(str),
            Err(e) => Err(BufferError::InvalidUtf8String(e)),
        }
    }

    pub fn pop_u32(&mut self) -> Result<u32, BufferError> {
        let bytes = self.pop_n_bytes(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn pop_f32(&mut self) -> Result<f32, BufferError> {
        let bytes = self.pop_n_bytes(4)?;
        Ok(f32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn pop_u8(&mut self) -> Result<u8, BufferError> {
        let bytes = self.pop_n_bytes(1)?;
        Ok(u8::from_le_bytes(bytes.try_into().unwrap()))
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn test_buffer_usize() {
        let mut buf = Buffer::new();
        buf.append_usize(5).unwrap();

        assert_eq!(5, buf.pop_usize().unwrap());
    }

    #[test]
    fn test_buffer_string() {
        let mut buf = Buffer::new();
        buf.append_string(&"test".to_string()).unwrap();

        assert_eq!("test", buf.pop_string().unwrap());
    }

    #[test]
    fn test_buffer_u32() {
        let mut buf = Buffer::new();
        buf.append_u32(7);

        assert_eq!(7, buf.pop_u32().unwrap());
    }

    #[test]
    fn test_buffer_f32() {
        let mut buf = Buffer::new();
        buf.append_f32(7.0);

        assert_eq!(7.0, buf.pop_f32().unwrap());
    }

    #[test]
    fn test_buffer_u8() {
        let mut buf = Buffer::new();
        buf.append_u8(15);

        assert_eq!(15, buf.pop_u8().unwrap());
    }

    #[test]
    fn test_buffer_buffer() {
        let mut buf = Buffer::new();
        let mut other = Buffer {
            v: vec![1,2,3,4],
        };

        buf.append(&mut other);

        assert_eq!(vec![1,2,3], buf.pop_n_bytes(3).unwrap());
    }
}