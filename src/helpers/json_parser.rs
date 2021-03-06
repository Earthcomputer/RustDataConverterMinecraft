use std::fmt::Write;
use std::str::FromStr;
use nom::error::Error;
use nom::{AsChar, Finish, IResult};
use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, is_a, tag};
use nom::character::complete::{char, satisfy, space1};
use nom::combinator::{map, map_opt, map_res, recognize, value};
use nom::multi::{many0, separated_list0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use rust_dataconverter_engine::{ListType, MapType, ObjectRef, ObjectType, Types};

pub(crate) fn stringify_map<T: Types + ?Sized>(map: T::Map) -> String {
    let mut str = String::new();
    stringify::<T>(&T::Object::create_map(map), &mut str).expect("Should not get Err writing to String");
    str
}

fn stringify<T: Types + ?Sized>(obj: &T::Object, str: &mut String) -> std::fmt::Result {
    match obj.as_ref() {
        ObjectRef::Byte(b) => write!(str, "{}", b)?,
        ObjectRef::Short(s) => write!(str, "{}", s)?,
        ObjectRef::Int(i) => write!(str, "{}", i)?,
        ObjectRef::Long(l) => write!(str, "{}", l)?,
        ObjectRef::Float(f) => write!(str, "{}", f)?,
        ObjectRef::Double(d) => write!(str, "{}", d)?,
        ObjectRef::ByteArray(arr) => {
            str.push('[');
            for (i, &b) in <&[i8]>::into_iter(arr).enumerate() {
                if i != 0 {
                    str.push(',');
                }
                write!(str, "{}", b)?;
            }
            str.push(']');
        }
        ObjectRef::ShortArray(arr) => {
            str.push('[');
            for (i, &s) in <&[i16]>::into_iter(arr).enumerate() {
                if i != 0 {
                    str.push(',');
                }
                write!(str, "{}", s)?;
            }
            str.push(']');
        }
        ObjectRef::IntArray(arr) => {
            str.push('[');
            for (i, &int) in <&[i32]>::into_iter(arr).enumerate() {
                if i != 0 {
                    str.push(',');
                }
                write!(str, "{}", int)?;
            }
            str.push(']');
        }
        ObjectRef::LongArray(arr) => {
            str.push('[');
            for (i, &l) in <&[i64]>::into_iter(arr).enumerate() {
                if i != 0 {
                    str.push(',');
                }
                write!(str, "{}", l)?;
            }
            str.push(']');
        }
        ObjectRef::List(list) => {
            str.push('[');
            for (i, obj) in list.iter().enumerate() {
                if i != 0 {
                    str.push(',');
                }
                stringify::<T>(obj, str)?;
            }
            str.push(']');
        }
        ObjectRef::Map(map) => {
            str.push('{');
            for (i, key) in map.keys().enumerate() {
                if i != 0 {
                    str.push(',');
                }
                stringify_string(key, str);
                str.push(':');
                stringify::<T>(map.get(&key[..]).unwrap(), str)?;
            }
            str.push('}');
        }
        ObjectRef::String(input) => stringify_string(input, str),
    }
    Ok(())
}

fn stringify_string(input: &str, output: &mut String) {
    output.push('"');
    for ch in input.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            _ => output.push(ch),
        }
    }
    output.push('"');
}

pub(crate) fn parse_map<T: Types + ?Sized>(json: &str) -> Result<T::Map, Error<&str>> {
    preceded(space, object::<T>)(json).finish().map(|(_, o)| o)
}

fn space(i: &str) -> IResult<&str, ()> {
    value((), many0(alt((space1, is_a("\r\n")))))(i)
}

fn any<T: Types + ?Sized>(i: &str) -> IResult<&str, T::Object> {
    alt((
        map(object::<T>, T::Object::create_map),
        map(array::<T>, T::Object::create_list),
        map(string, T::Object::create_string),
        map_res(
            terminated(|i| recognize_float(i), space),
            |str| {
                Result::<_, <f64 as FromStr>::Err>::Ok(
                    match str::parse::<i64>(str) {
                        Ok(long) => T::Object::create_long(long),
                        Err(_) => T::Object::create_double(str::parse::<f64>(str)?)
                    }
                )
            }
        ),
        map(pair(tag("false"), space), |_| T::Object::create_bool(false)),
        map(pair(tag("true"), space), |_| T::Object::create_bool(true)),
        map(pair(tag("null"), space), |_| T::Object::create_bool(false)),
    ))(i)
}

fn object<T: Types + ?Sized>(i: &str) -> IResult<&str, T::Map> {
    map(delimited(
        pair(char('{'), space),
        separated_list0(pair(char(','), space), separated_pair(string, pair(char(':'), space), any::<T>)),
        pair(char('}'), space)
    ), |vec| {
        let mut map = T::Map::create_empty();
        for (k, v) in vec {
            map.set(k, v);
        }
        map
    })(i)
}

fn array<T: Types + ?Sized>(i: &str) -> IResult<&str, T::List> {
    map(delimited(
        pair(char('['), space),
        separated_list0(pair(char(','), space), any::<T>),
        pair(char(']'), space)
    ), |vec| {
        let mut list = T::List::create_empty();
        for v in vec {
            list.add(v);
        }
        list
    })(i)
}

fn string(i: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        escaped_transform(
            satisfy(|ch| ch != '\\' && ch != '"' && ch != '\n' && ch != '\r'),
            '\\',
            alt((
                value('\\', char('\\')),
                value('"', char('"')),
                value('\'', char('\'')),
                value('\n', char('n')),
                value('\r', char('r')),
                value('\t', char('t')),
                map_opt(
                    preceded(char('u'), recognize(tuple((hex_digit, hex_digit, hex_digit, hex_digit)))),
                    |str| char::from_u32(u32::from_str_radix(str, 16).unwrap())
                ),
            ))
        ),
        pair(char('"'), space)
    )(i)
}

fn hex_digit(i: &str) -> IResult<&str, ()> {
    value((), satisfy(|ch| ch.is_hex_digit()))(i)
}

#[cfg(test)]
#[cfg(feature = "quartz_nbt")]
mod tests {
    use quartz_nbt::snbt;
    use rust_dataconverter_engine::QuartzNbtTypes;
    use super::parse_map;

    type TestTypes = QuartzNbtTypes;

    macro_rules! assert_nbt_eq {
        ($a:expr, $b:expr) => {
            assert_eq!($a.inner(), $b.inner())
        }
    }

    #[test]
    fn test_parse_object() {
        assert_nbt_eq!(
            parse_map::<TestTypes>(r#"{"foo": "bar", "baz": "quux"}"#).unwrap(),
            snbt::parse(r#"{"foo": "bar", "baz": "quux"}"#).unwrap()
        );
    }

    #[test]
    fn test_parse_array() {
        assert_nbt_eq!(
            parse_map::<TestTypes>(r#"{"foo": ["bar", "baz", "quux"]}"#).unwrap(),
            snbt::parse(r#"{"foo": ["bar", "baz", "quux"]}"#).unwrap()
        )
    }

    #[test]
    fn test_parse_int() {
        assert_nbt_eq!(
            parse_map::<TestTypes>(r#"{"foo": 123}"#).unwrap(),
            snbt::parse(r#"{"foo": 123L}"#).unwrap()
        )
    }

    #[test]
    fn test_parse_double() {
        assert_nbt_eq!(
            parse_map::<TestTypes>(r#"{"foo": 123.45}"#).unwrap(),
            snbt::parse(r#"{"foo": 123.45}"#).unwrap()
        )
    }

    #[test]
    fn test_whitespace() {
        assert_nbt_eq!(
            parse_map::<TestTypes>(r#" { "foo" : "bar" , "list" : [ "a" , "b" ] , "long" : 1 , "double" : 1.2 } "#).unwrap(),
            snbt::parse(r#"{"foo": "bar", "list": ["a", "b"], "long": 1L, "double": 1.2}"#).unwrap()
        )
    }

    #[test]
    fn test_string_escapes() {
        assert_nbt_eq!(
            parse_map::<TestTypes>(r#"{"foo": "\\\n\r\t\"\u0020"}"#).unwrap(),
            snbt::parse(r#"{"foo": "\\\n\r\t\" "}"#).unwrap()
        )
    }
}
