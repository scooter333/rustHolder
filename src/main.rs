use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, stdin, Write, stdout,};

fn main() {
    let tester = read_string(stdin());
    let map = get_freq(tester);
    write_output(stdout(),map);
}


fn read_string<R: Read>(mut reader: R) -> String{
    // let mut lines = BufReader::new(reader).lines();
    // let mut holder: Vec<String> = vec![];
    // while let Some(Ok(line)) = lines.next() {
    //     if let Ok(f) = line.parse(){
    //         holder.push(f);
    //     }
    // };
    // holder
    
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    buffer


}


fn get_freq(given: String)->HashMap<String, i32>{
    let mut freq_map = HashMap::new();

    for word in given.split_whitespace(){
        let count = freq_map.entry(word.to_owned()).or_insert(0);
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
        freq_map.insert("Hello",1 as i32);
        assert_write(
            "Hello: 1\n",
            freq_map);
    }

    fn assert_write(expected: &str, results: HashMap<&str, i32> ) {
        let mut writer = Cursor::new(vec![]);
        write_output(&mut writer, results);
        assert_eq!(expected, String::from_utf8(writer.into_inner()).unwrap());
    }
}

#[cfg(test)]
mod count_tests {
    use super::get_freq;

    #[test]
    fn hello_world_test(){
        let test_map = get_freq("Hello world");
        assert_eq!(test_map.get("Hello"),Some(&1));
        assert_eq!(test_map.get("world"),Some(&1));
    }

    #[test]
    fn bad_entry(){
        let test_map = get_freq("Hello world");
        assert_eq!(test_map.get("garbage"),None);

    }
}

