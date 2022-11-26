use urlencoding::decode_binary;

pub fn parse_qsl(qs: &[u8], separator: char) -> Box<[(String, String)]> {
    String::from_utf8(decode_binary(qs).into_owned())
        .unwrap()
        .split(separator)
        .map(|value| value.split_once('=').unwrap_or(("", "")))
        .map(|value| (value.0.to_string(), value.1.to_string()))
        .collect::<Vec<(String, String)>>()
        .into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ampersand_separator() {
        let result = parse_qsl("key=1&key=2&anotherKey=a&yetAnother=z".as_bytes(), '&');
        assert_eq!(
            result,
            vec![
                ("key".to_string(), "1".to_string()),
                ("key".to_string(), "2".to_string()),
                ("anotherKey".to_string(), "a".to_string()),
                ("yetAnother".to_string(), "z".to_string())
            ]
            .into_boxed_slice()
        );
    }

    #[test]
    fn test_semicolon_separator() {
        let result = parse_qsl("key=1;key=2;anotherKey=a;yetAnother=z".as_bytes(), ';');
        assert_eq!(
            result,
            vec![
                ("key".to_string(), "1".to_string()),
                ("key".to_string(), "2".to_string()),
                ("anotherKey".to_string(), "a".to_string()),
                ("yetAnother".to_string(), "z".to_string())
            ]
            .into_boxed_slice()
        );
    }
}
