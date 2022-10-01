fn calculation_reversepoland(s : &String) -> i32{
    let s_ope: Vec<&str> = s.split(" ").collect();

    let mut stack = vec![];

    println!("{:?}", s_ope);

    for str in s_ope.into_iter() {
        match str {
            "+" => {
                let operand_left = stack.pop().unwrap();
                let operand_right =  stack.pop().unwrap();
                let add = operand_right + operand_left;
                stack.push(add);
            },
            "-" => {
                let operand_left = stack.pop().unwrap();
                let operand_right =  stack.pop().unwrap();
                let dif = operand_right - operand_left;
                stack.push(dif);
            },
            "*" => {
                let operand_left = stack.pop().unwrap();
                let operand_right =  stack.pop().unwrap();
                let dif = operand_right * operand_left;
                stack.push(dif);
            },
            "/" => {
                let operand_left = if stack[stack.len()-1] != 0 {
                    stack.pop().unwrap()
                } else {
                    print!("0除算が行われました \n");
                    return 0;
                };
                let operand_right = stack.pop().unwrap();
                let dif = operand_right / operand_left;
                stack.push(dif);
            },
            _ => if str.parse::<i32>().is_ok(){
                stack.push(str.parse().unwrap())
            }
            else {
                print!("不正な値が入っています \n");
                return 0
            }
        } 
    }

    return stack.pop().unwrap()
}

fn main() {
    let query1 = "3 4 +".to_string();
    let query2 = "3 4 -".to_string();
    let query3 = "3 4 *".to_string();
    let query4 = "1 4 /".to_string();
    let query5 = "3 0 /".to_string();
    let query6 = "3 3 + 5 * 2 - 4 /".to_string();
    let query7 = "2/1".to_string();
}

fn op() -> Option<i32> {
    Some(12)
}