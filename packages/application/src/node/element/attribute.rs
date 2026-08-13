#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Attribute {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool)
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        match other {
            Self::Boolean(b) => self.eq(b),
            Self::Integer(i) => self.eq(i),
            Self::Number(n) => self.eq(n),
            Self::String(s) => self.eq(s)
        }
    }
}

impl Eq for Attribute {}

impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmp_string().partial_cmp(&other.cmp_string())
    }
}

impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_string().cmp(&other.cmp_string())
    }
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Integer(i) => i.to_string(),
            Self::Boolean(b) => if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
    }
}

impl Attribute {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::String(str) => match str.trim().to_ascii_lowercase().as_str() {
                "false"  => false,
                "" => false,
                "0" => false,
                _ => true
            },
            Self::Number(num) => *num != 0.0,
            Self::Integer(int) => *int != 0,
            Self::Boolean(b) => *b
        }
    }

    fn cmp_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Integer(i) => i.to_string(),
            Self::Boolean(b) => if *b {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::String(s) => s.clear(),
            Self::Number(n) => *n = 0.0,
            Self::Integer(i) => *i = 0,
            Self::Boolean(b) => *b = false
        }
    }

    pub(crate) fn format(&self, name:&str) -> String {
        match self {
            Self::Boolean(b) => if *b {
                name.to_string()
            } else {
                String::new()
            },
            Self::Integer(i) => fmt_helper(name, &i.to_string()),
            Self::Number(n) => fmt_helper(name, &n.to_string()),
            Self::String(s) => fmt_helper(name, s)
        }
    }
}

fn fmt_helper(name:&str, value:&str) -> String {
    let mut output = String::with_capacity(name.len() + value.len() + 5);
    
    output.push_str(name);
    output.push_str(":\"");
    output.push_str(value);
    output.push_str("\",\n");

    output

}

impl From<bool> for Attribute {
    fn from(value:bool) -> Self {
        Self::Boolean(value)
    }
}

impl PartialEq<bool> for Attribute {
    fn eq(&self, value:&bool) -> bool {
        self.is_truthy() == *value
    }
}

impl From<&str> for Attribute {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl PartialEq<&str> for Attribute {
    fn eq(&self, other: &&str) -> bool {
        self.to_string().eq(*other)
    }
}

impl From<char> for Attribute {
    fn from(value: char) -> Self {
        Self::String(String::from(value))
    }
}

impl PartialEq<char> for Attribute {
    fn eq(&self, other: &char) -> bool {
        let str = self.to_string();
        if str.len() == 1 && let Some(char) = str.chars().next() {
            char == *other
        } else {
            false
        }
    }
}

impl From<String> for Attribute {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl PartialEq<String> for Attribute {
    fn eq(&self, other: &String) -> bool {
        self.to_string().eq(other)
    }
}

impl From<u8> for Attribute {
    fn from(value: u8) -> Self {
        Self::Integer(value.into())
    }
}

impl PartialEq<u8> for Attribute {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as u8)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<u16> for Attribute {
    fn from(value: u16) -> Self {
        Self::Integer(value.into())
    }
}

impl PartialEq<u16> for Attribute {
    fn eq(&self, other: &u16) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as u16)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<u32> for Attribute {
    fn from(value: u32) -> Self {
        Self::Integer(value.into())
    }
}

impl PartialEq<u32> for Attribute {
    fn eq(&self, other: &u32) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as u32)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<u64> for Attribute {
    fn from(value: u64) -> Self {
        Self::Integer(value as i64)
    }
}

impl PartialEq<u64> for Attribute {
    fn eq(&self, other: &u64) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as u64)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<i8> for Attribute {
    fn from(value: i8) -> Self {
        Self::Integer(value.into())
    }
}

impl PartialEq<i8> for Attribute {
    fn eq(&self, other: &i8) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as i8)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<i16> for Attribute {
    fn from(value: i16) -> Self {
        Self::Integer(value.into())
    }
}

impl PartialEq<i16> for Attribute {
    fn eq(&self, other: &i16) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as i16)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<i32> for Attribute {
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl PartialEq<i32> for Attribute {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as i32)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<i64> for Attribute {
    fn from(value: i64) -> Self {
        Self::Integer(value.into())
    }
}

impl PartialEq<i64> for Attribute {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as i64)),
            Self::Integer(i) => i.eq(other),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<f32> for Attribute {
    fn from(value: f32) -> Self {
        Self::Number(value.into())
    }
}

impl PartialEq<f32> for Attribute {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as u8 as f32)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(&(*other as f64)),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}

impl From<f64> for Attribute {
    fn from(value: f64) -> Self {
        Self::Number(value.into())
    }
}

impl PartialEq<f64> for Attribute {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Self::Boolean(b) => other.eq(&(*b as u8 as f64)),
            Self::Integer(i) => i.eq(&(*other as i64)),
            Self::Number(n) => n.eq(other),
            Self::String(s) => other.to_string().eq(s)
        }
    }
}