use std::str;

///! # Simple String Builder
///! Super basic string builder based on my memory of Javas
///! Accepts Strings and &str
///! Some one probobly did this better

#[derive(Debug)]
pub struct Builder { 
    builder: Vec<u8> 
}


/// Trait to convert to byte vec implement this to allow you struct to be used by builder
pub trait ToBytes {
    fn to_bytes(self) -> Vec<u8>;
}

impl ToBytes for String {
    fn to_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl ToBytes for &str {
    fn to_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}


/// 
/// # Example
/// 
/// ```
/// let name = "quinn".to_string();
/// let mut b = Builder::new();
/// b.append("Hello".to_string());
/// b.append(" My Name Is".to_string());
/// b.append(format!(" {} and I Like you 😍", name));
/// ```
impl Builder {
    pub fn new() -> Self {
        Builder {
            builder: Vec::default()
        }
    }

    pub fn append<B: ToBytes>(&mut self, buf: B) {
        self.builder.append(&mut buf.to_bytes())
    }

    pub fn to_string(&self) -> Result<String, str::Utf8Error> {
        let string_maybe = str::from_utf8(&self.builder);
        match string_maybe {
            Ok(s) => Ok(s.to_owned()),
            Err(e) => Err(e)
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Builder;

    #[test]
    fn test_string() {
        let name = "quinn".to_string();
        let mut b = Builder::new();
        b.append("Hello".to_string());
        b.append(" My Name Is".to_string());
        b.append(format!(" {} and I Like you 😍", name));

        assert_eq!(b.to_string().unwrap(), "Hello My Name Is quinn and I Like you 😍".to_owned())
    }
}



