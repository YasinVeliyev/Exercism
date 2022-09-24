pub fn encode(source: &str) -> String {
    if source == "" {
        return source.to_owned();
    }
    let source = source.chars().collect::<Vec<char>>();

    let mut s = source[0];
    let mut count = 0;
    let mut encoded_str = String::new();

    for i in source {
        if s == i {
            count += 1
        } else {
            if count != 1 {
                encoded_str += &format!("{}", count);
            }
            encoded_str.push(s);
            s = i;
            count = 1;
        }
    }

    if count != 1 {
        encoded_str += &format!("{}", count);
    }
    encoded_str.push(s);

    encoded_str
}

pub fn decode(source: &str) -> String {
    let source = source.split("").collect::<Vec<&str>>();
    let mut count = 0;
    let mut decoded_str = String::new();
    for i in source {
        match i.parse::<usize>() {
            Ok(n) => {
                if count != 0 {
                    count = count * 10 + n;
                } else {
                    count = n
                }
            }
            Err(_) => {
                let repeat = if count > 0 { count } else { 1 };
                decoded_str.extend(std::iter::repeat(i).take(repeat));
                count = 0
            }
        }
    }
    return decoded_str;
}
