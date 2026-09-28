use std::fmt;

use super::{Element, Attribute, Node};

#[derive(Debug, Clone)]
pub enum AttributeSelector<'a> {
    Has(&'a str),
    EqStrong(&'a str, &'a str),
    EqWeak(&'a str, &'a str),
    ListHasStrong(&'a str, &'a str),
    ListHasWeak(&'a str, &'a str),
    HasStrong(&'a str, &'a str),
    HasWeak(&'a str, &'a str),
    StartsWithStrong(&'a str, &'a str),
    StartsWithWeak(&'a str, &'a str),
    EndsWithStrong(&'a str, &'a str),
    EndsWithWeak(&'a str, &'a str),
    EqOrPrefixStrong(&'a str, &'a str),
    EqOrPrefixWeak(&'a str, &'a str),
}

impl<'a> AttributeSelector<'a> {
    fn match_attribute(&self, name:&str, value:&Attribute) -> bool {
        match self {
            Self::Has(n) => *n == name,
            Self::EqStrong(n, v) => *n == name
                && value.eq(v),
            Self::EqWeak(n, v) => *n == name
                && value.eq_weak(v),
            Self::ListHasStrong(n, v) => *n == name
                && value.list_has(v),
            Self::ListHasWeak(n, v) => *n == name
                && value.list_has_weak(v),
            Self::HasStrong(n, v) => *n == name
                && value.includes(v),
            Self::HasWeak(n, v) => *n == name
                && value.includes_weak(v),
            Self::StartsWithStrong(n, v) => *n == name
                && value.starts_with(v),
            Self::StartsWithWeak(n, v) => *n == name
                && value.starts_with_weak(v),
            Self::EndsWithStrong(n, v) => *n == name
                && value.ends_with(v),
            Self::EndsWithWeak(n, v) => *n == name
                && value.ends_with_weak(v),
            Self::EqOrPrefixStrong(n, v) => *n == name
                && value.eq_or_prefix(v),
            Self::EqOrPrefixWeak(n, v) => *n == name
                && value.eq_or_prefix_weak(v)
        }
    }

    fn match_element(&self, elm:&Element) -> bool {
        elm.attributes()
            .find(|(name, att)|self.match_attribute(name, att))
            .is_some()
    }
}

#[derive(Debug, Clone)]
pub enum QueryToken<'a> {
    TagName(&'a str),
    Id(&'a str),
    Class(&'a str),
    Attribute(AttributeSelector<'a>)
}

impl<'a> QueryToken<'a> {
    pub(crate) fn match_element(&self, elm:&Element) -> bool {
        match self {
            Self::TagName(name) => (&elm.tag_name()).eq(name),
            Self::Id(id) => elm.get_id().eq(id),
            Self::Class(class_name) => elm.get_class_name().eq(class_name),
            Self::Attribute(sel) => sel.match_element(elm)
        }
    }

    fn make(flag:u8, value:&'a str) -> Result<Self, QueryError> {
        match flag {
            TAG_NAME_FLAG => Ok(Self::TagName(value)),
            CLASS_NAME_FLAG => Ok(Self::Class(value)),
            ID_FLAG => Ok(Self::Id(value)),
            _ => {
                todo!("Parse Attribute String")
            }
        }
    }
}

#[derive(Debug)]
pub enum QueryError {
    InvalidOperator(String),
    InvalidFlag(String),
    ParseError(String, usize)
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperator(op) => write!(f, "Invalid Operator: {}!", op),
            Self::InvalidFlag(flag) => write!(f, "Invalid Flag: {}!", flag),
            Self::ParseError(string, idx) => write!(f, "Error parsing \"{}\" at {}!", string, idx)
        }
    }
}

const TAG_NAME_FLAG:u8 = 1;
const CLASS_NAME_FLAG:u8 = 2;
const ID_FLAG:u8 = 4;
const ATTRIBUTE_FLAG:u8 = 8;
const ATTRIBUTE_END:u8 = 15;

fn check_state(current:u8, flags:&mut u8) -> bool {
    if current == ATTRIBUTE_END || *flags & ATTRIBUTE_END  == ATTRIBUTE_END {
        false
    } else {
        *flags |= current;
        true
    }
}

fn next<'a, 'b>(string:&'a str, start:&'b mut usize, idx:&'b mut usize, flags:&'b mut u8, current:&'b mut u8) -> Result<Option<QueryToken<'a>>, QueryError> {
    let length = string.len();

    if *idx >= length {
        return Ok(None);
    }

    while *idx < length {
        let mut value:Option<QueryToken<'_>> = None;

        match &string[*idx..*idx+1] {
            "#" => if *current >= ATTRIBUTE_FLAG || *flags & ID_FLAG == ID_FLAG {
                return Err(QueryError::ParseError(string.to_string(), *idx))
            } else {
                if check_state(*current, flags) {
                    value = Some(QueryToken::make(*current, &string[*start..*idx])?);
                }

                *current = ID_FLAG;
            },
            "*" => if *current >= ATTRIBUTE_FLAG || *flags & CLASS_NAME_FLAG == CLASS_NAME_FLAG {
                if check_state(*current, flags) {
                    value = Some(QueryToken::make(*current, &string[*start..*idx])?);
                }

                *current = CLASS_NAME_FLAG;
            },
            "[" => if *current < ATTRIBUTE_FLAG {
                if check_state(*current, flags) {
                    value = Some(QueryToken::make(*current, &string[*start..*idx])?);
                }

                *current = ATTRIBUTE_FLAG;
            },
            "]" => if *current != ATTRIBUTE_FLAG {
                return Err(QueryError::ParseError(string.to_string(), *idx))
            } else {
                if check_state(*current, flags) {
                    value = Some(QueryToken::make(*current, &string[*start..*idx])?);
                }
            },
            _ => {}
        }

        *idx += 1;

        if value.is_some() {
            *start = *idx;
            return Ok(value)
        }
    }

    let value: QueryToken<'_> = QueryToken::make(*current,&string[*start..*idx])?;
    *current = ATTRIBUTE_END;
    *flags |= ATTRIBUTE_END;
    *start = *idx;
    
    Ok(Some(value))
}

pub trait Query {
    fn parse(&self) -> Result<Vec<QueryToken<'_>>, QueryError>;

    fn match_element(&self, elm:&Element) -> Result<bool, QueryError> {
        for token in &(self.parse()?) {
            if !token.match_element(elm) {
                return Ok(false)
            }
        }

        Ok(true)
    }
}

fn parse_string(string:&str) -> Result<Vec<QueryToken<'_>>, QueryError> {
    let mut vec:Vec<QueryToken<'_>> = Vec::new();

    let mut start:usize = 0;
    let mut idx:usize = 0;
    let mut flags:u8 = 0;
    let mut current:u8 = 0;

    while let Some(token) = next(string, &mut start, &mut idx, &mut flags, &mut current)? {
        vec.push(token);
    }

    Ok(vec)
}

impl<'a> Query for Vec<QueryToken<'a>> {
    fn parse(&self) -> Result<Vec<QueryToken<'_>>, QueryError> {
        Ok(self.to_vec())
    }
}

impl<'a> Query for &str {
    fn parse(&self) -> Result<Vec<QueryToken<'_>>, QueryError> {
        parse_string(self)
    }
}

impl<'a> Query for String {
    fn parse(&self) -> Result<Vec<QueryToken<'_>>, QueryError> {
        parse_string(self)
    }
}