
/// General trait that each object must implemnet 
/// This is the foundation of this recursive parser.
pub trait Parse {
    /// Parse the object and return the unused data.
    fn parse(data: &[u8]) -> (&[u8], Self);
}

/// This trait is needed on ambiguous object so we can decide which object shall pare the data.
pub trait CanParse {
    /// If the current class can parse an object form the current data
    fn can_parse(data: &[u8]) -> bool;
}

#[macro_export]
/// Return the next char and update the given data pointer.
macro_rules! next_char {
    ($data:ident) => {{
        let result = $data[0];
        $data = &$data[1..];
        result
    }};
}

#[macro_export]
/// Simple macro which pourpouse is mainly to clean the code from redundant 
/// assignements. It just call the parse method and update the data pointer.
macro_rules! parse {
    ($data:ident, $type:ty) => {{
        let (tmp_data, result) = <$type>::parse(skip_whitespace($data));
        $data = skip_whitespace(tmp_data);
        result
    }};
}

/// This is a general function used usually to match parenthesis.
/// It returns the reminder data and the 
pub fn get_next_matching(data: &[u8], start_char: u8, end_char: u8) -> (&[u8], &[u8]) {
    // every time we encounter a start_char this is increased
    // and every time a end_char is enconutered is decreased.
    // so if the counter is 0, we know that we are at the end of the well parenthesized
    // expression.
    let mut counter = 0;

    // The length of the matching substring
    let mut index = 0;

    // byte pointer that is increased at each cycle 
    let mut tmp_data = data;
    while !data.is_empty() {

        // get the next char
        let char = next_char!(tmp_data);

        // Update the counter
        if char == start_char {
            counter += 1;
        }
        if char == end_char {
            counter -= 1;
        }

        // 
        index += 1;

        // If the counter is 0 it means that we encountered the same number of
        // open and closed parenthesis so we can end.
        if counter == 0{
            break;
        }
    } 
    // return the REMAINDER and then the MATCHING
    (&data[index..], &data[1..index - 1])
}

/// Skip to the next char not in "\n\r\t "
pub fn split_at(data: &[u8], delimiter: u8) -> (&[u8], &[u8]) {
    let mut idx = 0;
    while idx < data.len() && data[idx] != delimiter {
        idx += 1;
    }
    // skip the delimiter:
    (&data[std::cmp::min(idx + 1, data.len() - 1)..], &data[..idx])
}

/// Skip to the next char not in "\n\r\t "
pub fn skip_whitespace(mut data: &[u8]) -> &[u8] {
    while !data.is_empty() && b"\n\r\t ".contains(&data[0]){
        data = &data[1..];
    }
    data
}

pub fn bytes_to_string(data: &[u8]) -> String{
    String::from_utf8(data.to_vec()).unwrap()
}

pub fn trim(mut data: &[u8]) -> &[u8] {
    while !data.is_empty() && b"\n\r\t ".contains(&data[0]){
        data = &data[1..];
    }

    while !data.is_empty() && b"\n\r\t ".contains(&data[data.len().saturating_sub(1)]){
        data = &data[..data.len().saturating_sub(2)];
    }

    data
}

pub fn trim_str(mut data: String) -> String {
    while !data.is_empty() && b"\n\r\t ".contains(&data.as_bytes()[0]){
        data.remove(0);
    }

    while !data.is_empty() && b"\n\r\t ".contains(&data.as_bytes()[data.len().saturating_sub(1)]){
        data.pop();
    }

    data
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_next_matching() {
        let data = "{a{aaa}b{{c}}}test".to_string();
        let (reminder, matching) = get_next_matching(data.as_bytes(), b'{', b'}');
        assert_eq!(reminder, "test".as_bytes());
        assert_eq!(matching, "a{aaa}b{{c}}".as_bytes());
    }

    #[test]
    fn test_next_char() {
        let data = "abcdefg".to_string();
        let mut ptr = data.as_bytes();
        assert_eq!(next_char!(ptr), b'a');
        assert_eq!(next_char!(ptr), b'b');
        assert_eq!(next_char!(ptr), b'c');
        assert_eq!(next_char!(ptr), b'd');
        assert_eq!(next_char!(ptr), b'e');
        assert_eq!(next_char!(ptr), b'f');
        assert_eq!(next_char!(ptr), b'g');
    }

    #[test]
    fn test_skip_whitespace() {
        let data = "abcdefg".to_string();
        let ptr = skip_whitespace(data.as_bytes());
        assert_eq!(ptr, data.as_bytes());

        let data_with_stuff = "\r\n\t abcdefg".to_string();
        let ptr = skip_whitespace(data_with_stuff.as_bytes());
        assert_eq!(ptr, data.as_bytes());
    }

    #[test]
    fn test_split_at() {
        let data = "AA,BB,CC,DD".to_string();
        let mut ptr = data.as_bytes();
        let (ptr, first)  = split_at(ptr, b',');
        assert_eq!(first, b"AA");
        let (ptr, second) = split_at(ptr, b',');
        assert_eq!(second, b"BB");
        let (ptr, third)  = split_at(ptr, b',');
        assert_eq!(third, b"CC");
        let (ptr, forth)  = split_at(ptr, b',');
        assert_eq!(forth, b"DD");
    }
}