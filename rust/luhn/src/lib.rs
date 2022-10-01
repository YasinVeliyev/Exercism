pub fn is_valid(code: &str) -> bool {
    let mut index = 0;
    let mut sum = 0;
    if code.contains("-"){
         return false
    }
    let code = code.replace(" ","");
    for (i,k)  in code.trim().chars().rev().enumerate() {
        match i.to_string().to::<i32>() {
            Ok(n) => {
                if index  % 2 == 0 {
                    if n % 9 == 0 && n != 0 {
                        sum += 9
                    } else {
                        sum += n * 2 % 9
                    }
                } else {
                    sum += n
                }
            }
            Err(_) => return false,
        }
    }
    if index <= 1 {
        return false;
    };
    sum % 10 == 0 && sum >= 0
}
