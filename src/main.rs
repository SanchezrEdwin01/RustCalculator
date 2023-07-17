use regex::Regex;

fn main() {
    
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_res = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    println!("Por favor introduce tu expresion: ");
    let mut expression= String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    loop {

        let caps = re_mul.captures(expression.as_str());

        if caps.is_none(){
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let multi: i32 = left_value * right_value;

        expression = expression.replace(cap_expression, &multi.to_string());

    }

    loop {

        let caps = re_div.captures(expression.as_str());

        if caps.is_none(){
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let div: i32 = left_value / right_value;

        expression = expression.replace(cap_expression, &div.to_string());

    }

    loop {

        let caps = re_add.captures(expression.as_str());

        if caps.is_none(){
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition: i32 = left_value + right_value;

        expression = expression.replace(cap_expression, &addition.to_string());

    }

    loop {

        let caps = re_res.captures(expression.as_str());

        if caps.is_none(){
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let rest: i32 = left_value - right_value;

        expression = expression.replace(cap_expression, &rest.to_string());

    }


    println!("Resultado: {}", expression);

}
