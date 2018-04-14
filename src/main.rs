use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, stdin, Write, stdout,};

fn main() {
    transform(stdin(),stdout());
}

fn transform<R: Read, W: Write>(input: R, output: W) {
    let tester = read_string(input);
    let count = get_freq(tester);
    write_output(output, count);
}

fn read_string<R: Read>( reader: R) -> String{
    let given_lines = BufReader::new(reader).lines();
    let mut buffer = String::new();
    for line in given_lines {
        buffer = line.unwrap();
        break;
    }
    buffer
}


fn get_freq(given: String)->HashMap<String, i32>{
    let mut freq_map = HashMap::new();

    for word in given.split_whitespace(){
        let count = freq_map.entry(word.to_owned().to_lowercase().replace(",","")
            .replace(".","").replace("!","").replace("?","")).or_insert(0);
        *count += 1;
    }

    freq_map
}


fn write_output<W: Write>(mut writer: W, map:HashMap<String, i32> ){
    if map.is_empty(){
        write!(writer, "Nothing given.\n").unwrap();
    }
        else {
            for (key, value) in &map {
                write!(writer, "{}: {}\n", key, value).unwrap();
            }
        }
}

#[cfg(test)]
mod read_measurements_tests {
    use super::read_string;
    use std::io::Cursor;

    #[test]
    fn read_hello_world() {
        assert_read("Hello world".to_string(), "Hello world\n");
    }

    fn assert_read(expected: String, input: &str) {
        let mock_read = Cursor::new(input);
        let count = read_string(mock_read);
        assert_eq!(expected, count);
    }
}



#[cfg(test)]
mod write_output_tests {
    use super::write_output;
    use std::io::Cursor;
    use std::collections::HashMap;

    #[test]
    fn no_measurements_output() {
        assert_write("Nothing given.\n",
                     HashMap::new());
    }

    #[test]
    fn some_measurements_output() {
        let mut freq_map = HashMap::new();
        freq_map.insert("hello".to_string(),1 as i32);
        assert_write(
            "hello: 1\n",
            freq_map);
    }

    fn assert_write(expected: &str, results: HashMap<String, i32> ) {
        let mut writer = Cursor::new(vec![]);
        write_output(&mut writer, results);
        assert_eq!(expected, String::from_utf8(writer.into_inner()).unwrap());
    }
}

#[cfg(test)]
mod transform_tests {
    use super::transform;
    use std::io::Cursor;

    #[test]
    fn no_input() {
        assert_transform("", "Nothing given.\n");
    }

    #[test]
    fn input_hello_world() {
        assert_transform(
            "Hello",
            "hello: 1\n");
    }
    #[test]
    fn input_punc_hello_world() {
        assert_transform(
            "Hello!",
            "hello: 1\n");
    }

    fn assert_transform(input: &str, expected_output: &str) {
        let input = Cursor::new(input);
        let mut output = Vec::new();
        transform(input, &mut output);
        let output_string = String::from_utf8(output).unwrap();
        assert_eq!( output_string, expected_output );
    }
}


#[cfg(test)]
mod count_tests {
    use super::get_freq;

    #[test]
    fn hello_world_test(){
        let test_map = get_freq("Hello world".to_string());
        assert_eq!(test_map.get("hello"),Some(&1));
        assert_eq!(test_map.get("world"),Some(&1));
    }

    #[test]
    fn hello_world_punc_test(){
        let test_map = get_freq("Hello world!".to_string());
        assert_eq!(test_map.get("hello"),Some(&1));
        assert_eq!(test_map.get("world"),Some(&1));
    }

    #[test]
    fn bad_entry(){
        let test_map = get_freq("Hello world".to_string());
        assert_eq!(test_map.get("garbage"),None);

    }

    #[test]
    fn many_word_test(){
        let test_map = get_freq("hello hello world world".to_string());
        assert_eq!(test_map.get("hello"),Some(&2));
        assert_eq!(test_map.get("world"),Some(&2));
    }

    #[test]
    fn case_test(){
        let test_map = get_freq("Hello hello World world".to_string());
        assert_eq!(test_map.get("hello"),Some(&2));
        assert_eq!(test_map.get("world"),Some(&2));
        assert_eq!(test_map.get("Hello"),None);
        assert_eq!(test_map.get("World"),None);
    }
}
