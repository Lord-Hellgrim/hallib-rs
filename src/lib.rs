use std::{num::{ParseFloatError, ParseIntError}, str::Utf8Error};

#[derive(Clone, Copy, Hash, PartialEq)]
pub struct KeyString {
    inner: [u8;64],
}

impl std::fmt::Debug for KeyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyString").field("inner", &self.as_str()).finish()
    }
}

impl std::fmt::Display for KeyString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = bytes_to_str(&self.inner).expect(&format!("A KeyString should always be valid utf8.\nThe KeyString that was just attempted to Display was:\n{:x?}", self.inner));
        write!(f, "{}", text)
    }   
}

impl Default for KeyString {
    fn default() -> Self {
        Self { inner: [0;64] }
    }
}

/// Turns a &str into a KeyString. If the &str has more than 64 bytes, the last bytes will be cut.
impl From<&str> for KeyString {
    fn from(s: &str) -> Self {

        let mut inner = [0u8;64];

        let mut min = std::cmp::min(s.len(), 64);
        inner[0..min].copy_from_slice(&s.as_bytes()[0..min]);

        loop {
            if min == 0 {break}
            match std::str::from_utf8(&inner[0..min]) {
                Ok(_) => break,
                Err(_) => min -= 1,
            }
        }

        KeyString {
            inner
        }

    }
}


impl TryFrom<&[u8]> for KeyString {
    type Error = Utf8Error;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        let mut inner = [0u8;64];

        let min = std::cmp::min(s.len(), 64);
        inner[0..min].copy_from_slice(&s[0..min]);

        match std::str::from_utf8(&inner) {
            Ok(_) => {
                Ok(KeyString {inner})
            },
            Err(e) => Err(e),
        }
    }
}

impl Eq for KeyString {}

impl Ord for KeyString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for KeyString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl KeyString {

    pub fn new() -> Self {
        KeyString {
            inner: [0u8; 64]
        }
    }

    pub fn len(&self) -> usize {
        let mut output = 0;
        for byte in self.inner {
            match byte {
                0 => break,
                _ => output += 1,
            }
        }
        output
    }

    pub fn push(&mut self, s: &str) {

        if self.len() + s.len() > 64 {
            return
        }

        let mut end_index = 0;
        for (index, byte) in self.inner.iter().enumerate() {
            if byte == &0 {
                end_index = index+1;
            }
        }

        for (index, byte) in s.as_bytes().iter().enumerate() {
            self.inner[index+end_index] = *byte;
        }

    }

    pub fn as_str(&self) -> &str {
        // This is safe since an enforced invariant of KeyString is that it is utf8
        unsafe { std::str::from_utf8_unchecked(&self.inner[0..self.len()]) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.inner[0..self.len()]
    }

    pub fn raw(&self) -> &[u8] {
        &self.inner
    }

    /// These functions may panic and should only be called if you are certain that the KeyString contains a valid number
    pub fn to_i32(&self) -> i32 {
        self.as_str().parse::<i32>().unwrap()
    }

    /// These functions may panic and should only be called if you are certain that the KeyString contains a valid number
    pub fn to_f32(&self) -> f32 {
        self.as_str().parse::<f32>().unwrap()
    }

    pub fn to_i32_checked(&self) -> Result<i32, ParseIntError> {
        self.as_str().parse::<i32>()
    }

    pub fn to_f32_checked(&self) -> Result<f32, ParseFloatError> {
        self.as_str().parse::<f32>()
    }

}


/// Removes the trailing 0 bytes from a str created from a byte buffer
pub fn bytes_to_str(bytes: &[u8]) -> Result<&str, Utf8Error> {
    let mut index: usize = 0;
    let len = bytes.len();
    let mut start: usize = 0;
    
    while index < len {
        if bytes[index] != 0 {
            break
        }
        index += 1;
        start += 1;
    }

    if bytes.is_empty() {
        return Ok("")
    }

    if start >= bytes.len()-1 {
        return Ok("")
    }

    let mut stop: usize = start;
    while index < len {
        if bytes[index] == 0 {
            break
        }
        index += 1;
        stop += 1;
    }

    std::str::from_utf8(&bytes[start..stop])
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
