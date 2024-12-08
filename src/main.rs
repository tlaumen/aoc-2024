use std::{array, clone, fs, str::FromStr, vec};
use regex::Regex;

fn main() {
    day_5();
}

fn day_1() {
    let rdr = csv::Reader::from_path("day1.csv");
    let reader = match rdr {
        Ok(reader) => reader,
        Err(err) => panic!("The reader has failed"),
    };
    
    // for res in rdr.expect("a line in csv file").records() {
    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];

    for record in reader.into_records() {
        let record = match record {
            Ok(record) => record,
            Err(err) => panic!("An error has occured while parsing"),
        };

        let line = record.iter().map(|field| field.trim().to_string());
        let mut i: i8 = 0;
        for l in line{
            let l = l.split("   ");
            for part in l {
                if i == 0 {
                    v1.push(part.parse().expect("Not a integer"));
                } else {
                    v2.push(part.parse().expect("Not a integer"));
                }
                i += 1;
            };

        };
    };

    // Part 1: sum all differences with lists sorted
    v1.sort();
    v2.sort();
    let mut s: i32 = 0;
    for (l1, l2) in v1.iter().zip(v2.iter()) {
        let diff: i32 = l2 - l1;
        s += diff.abs();
    };
    println!("{}", s);

    // Part 2: find number of occurances of elements of v1 in v2 and sum product of number of occurences and number itself
    let mut s2 = 0;
    for n1 in v1.iter(){
        // let n2 = n1 as usize;
        let n_occurences =  v2.iter().filter(|&x| x == n1).count() as i32;
        s2 += n1 * n_occurences;
    }
    println!("{}", s2);
}


fn is_safe(v: &Vec<i32>) -> bool {
    let v1 = &v[..v.len()-1];
    let v2 = &v[1..];
    let mut safe: bool = true;
    let mut i: i8 = 0;
    for (n1, n2) in v1.iter().zip(v2.iter()){
        if v1[v1.len()-1] > v1[0]{ 
            if ! safe_number_combo(n1, n2) {
                safe = false;
            }
        } else {
            if ! safe_number_combo(n2, n1) {
                safe = false;
            }
        };
        i += 1;
    };
    return safe
}

fn get_unsafe_indices(v: &Vec<i32>) -> Vec<usize> {
    let v1 = &v[..v.len()-1];
    let v2 = &v[1..];
    let mut unsafe_indices: Vec<usize> = vec![];
    let mut i: usize = 0;
    for (n1, n2) in v1.iter().zip(v2.iter()){
        if v1[v1.len()-1] > v1[0]{ 
            if ! safe_number_combo(n1, n2) {
                unsafe_indices.push(i);
                unsafe_indices.push(i+1);
            }
        } else {
            if ! safe_number_combo(n2, n1) {
                unsafe_indices.push(i);
                unsafe_indices.push(i+1);
            }
        };
        i += 1;
    };
    return unsafe_indices
    
}

fn safe_number_combo(n1: &i32, n2: &i32) -> bool {
    if n2 - n1 > 3 {
        return false
    } else if n1 >= n2 {
        return false
    } else {
        return true
    }
}

fn get_filtered_vector(v: &Vec<i32>, idx: usize) -> Vec<i32> {
    // println!("before filter: {:?}", v);
    let mut new_v: Vec<i32> = vec![];
    for (i, item) in v.iter().enumerate() {
        if i != idx {
            new_v.push(*item)
        }
    };
    // println!("after filter: {:?}", new_v);
    new_v
}

fn day_2() {
    let rdr = csv::Reader::from_path("day2.csv");
    let reader = match rdr {
        Ok(reader) => reader,
        Err(err) => panic!("The reader has failed"),
    };
    

    let mut codes: Vec<Vec<i32>> = vec![];
    for record in reader.into_records() {
        let record = match record {
            Ok(record) => record,
            Err(err) => panic!("An error has occured while parsing"),
        };
        let mut code_line: Vec<i32> = vec![];
        let line = record.iter().map(|field| field.trim().to_string());
        for l in line{
            let l = l.split(" ");
            for n in l {
                code_line.push(n.parse().expect("Not an integer"));
            };
        };
        codes.push(code_line);
    };
    
    //Part 1: count number of safe codes
    let mut n_safe = 0;
    for code_line in codes.iter(){
        let safe = is_safe(code_line);
       if safe {
            n_safe += 1;
        } else {
            println!("Code line is not safe: {:?}", code_line);
            // println!("Unsafe indices are: {:?}", unsafe_indices);
        };
    };
    println!("Numer of safe codes: {:}", n_safe);

    // Part 2: count number of safe codes, 1 unsafe value can be omitted
    let mut n_safe = 0;
    for code_line in codes.iter(){
        let safe = is_safe(code_line);
        let unsafe_indices = get_unsafe_indices(code_line);
       if safe {
            n_safe += 1;
        } else {
            // println!("Code line is not safe: {:?}", code_line);
            // println!("Unsafe indices are: {:?}", unsafe_indices);
            for idx in unsafe_indices.iter() {
                let v = &get_filtered_vector(code_line, *idx);
                let safe = is_safe(v);
                if safe {
                    n_safe += 1;
                    println!("Found another safe combination: {:?}", v);
                    break;
                } else {
                    println!("Non safe combination : {:?}", v);
                }
            } 
        };
    };
    println!("Numer of safe codes: {:}", n_safe);
}

fn extract_mul(s: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut results = vec![];
    let res = re.find_iter(s);
    for r in res{
        results.push(r.as_str());
        // println!("{:?}", r2);
    };
    results
}

fn extract_mul_do_dont(s: &str) -> Vec<&str> {
    let mut results = vec![];
    let s1: Vec<&str> = s.split("do()").collect();
    for el in s1{
        let s2: Vec<&str> = el.split("don't()").collect();
        let mut muls = extract_mul(s2[0]);
        results.append(&mut muls);
    };
    results
}

fn parse_numbers(s: &str) -> (i32, i32){
    let sl: &str = &s[4..s.len()-1];
    let numstr = sl.split(",");
    let mut nums: Vec<i32> = vec![];
    for n in numstr {
        nums.push(FromStr::from_str(n).unwrap());
    }
    (nums[0], nums[1])
}

fn day_3() {
    let s = fs::read_to_string("day3.txt").expect("Parsing of file went wrong");

    //Part 1: extract mul naively
    let muls: Vec<&str> = extract_mul(&s);
    let  mut total = 0;
    for mul in muls {
        let tup = parse_numbers(mul);
        total += tup.0 * tup.1;
    }
    println!("{:?}", total);
    let mut total = 0;
    let muls = extract_mul_do_dont(&s);
    for mul in muls{
        let tup = parse_numbers(mul);
        total += tup.0 * tup.1;
    }
    println!("{:?}", total);

}

fn from_str_to_matrix(s: &str)  -> Vec<Vec<char>> {
    let lines: Vec<&str> = s.split("\n").collect();
    let mut m: Vec<Vec<char>> = vec![];
    for line in lines {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        };
        m.push(row);
    };
    println!("{:?}", m);
    m
}

fn dims_matrix(m: Vec<Vec<char>>) -> (usize, usize) {
    (m.len(), m[0].len())

}

fn display_xmas_matrix(m: Vec<Vec<char>>, indices: Vec<(usize, usize)>) {
    for (i, line) in m.iter().enumerate(){
        for (j, c) in line.into_iter().enumerate(){
            if indices.contains(&(i, j)) {
                print!("{:}", c);
            } else {
                print!(".");
            }
        };
        print!("\n")
    }; 
}

fn search_horizontal(m: Vec<Vec<char>>, i: usize, j: usize) -> (bool, Vec<(usize, usize)>) {
    let s_target = String::from_str("XMAS").expect("XMAS string cannot be created");
    let mut s = String::new();
    let dims= dims_matrix(m.clone());
    let j_max = dims.1;
    if j + s_target.len() > j_max {
        return (false, vec![(0, 0)]);
    };
    let mut indices: Vec<(usize, usize)> = vec![];
    for jj in j .. j+s_target.len() {
        s.push(m.clone()[i][jj]);
        indices.push((i, jj))
    };
    if s.eq(&s_target) {
        return (true, indices);
    } else {
        return (false, indices);
    };
}
fn search_horizontal_back(m: Vec<Vec<char>>, i: usize, j: usize) -> (bool, Vec<(usize, usize)>) {
    let s_target = String::from_str("XMAS").expect("XMAS string cannot be created");
    let mut s = String::new();
    if j < s_target.len() -1 {
        return (false, vec![(0, 0)]);
    };
    let mut indices: Vec<(usize, usize)> = vec![];
    for jj in (j+1-s_target.len().. j+1).rev() {
        s.push(m.clone()[i][jj]);
        indices.push((i, jj))
    };
    if s.eq(&s_target) {
        return (true, indices);
    } else {
        return (false, indices);
    };
}
fn search_vertical(m: Vec<Vec<char>>, i: usize, j: usize) -> (bool, Vec<(usize, usize)>) {
    let s_target = String::from_str("XMAS").expect("XMAS string cannot be created");
    let mut s = String::new();
    let dims= dims_matrix(m.clone());
    let i_max = dims.0;
    if i + s_target.len() > i_max {
        return (false, vec![(0, 0)]);
    };
    let mut indices: Vec<(usize, usize)> = vec![];
    for ii in i .. i+s_target.len() {
        s.push(m.clone()[ii][j]);
        indices.push((ii, j))
    };
    if s.eq(&s_target) {
        return (true, indices);
    } else {
        return (false, indices);
    };
}
fn search_vertical_back(m: Vec<Vec<char>>, i: usize, j: usize) -> (bool, Vec<(usize, usize)>) {
    let s_target = String::from_str("XMAS").expect("XMAS string cannot be created");
    let mut s = String::new();
    if i < s_target.len() -1 {
        return (false, vec![(0, 0)]);
    };
    let mut indices: Vec<(usize, usize)> = vec![];
    for ii in (i+1-s_target.len() .. i+1).rev() {
        s.push(m.clone()[ii][j]);
        indices.push((ii, j))
    };
    if s.eq(&s_target) {
        return (true, indices);
    } else {
        return (false, indices);
    };
}
fn search_diagonal_down_right(m: Vec<Vec<char>>, i: usize, j: usize) -> (bool, Vec<(usize, usize)>) {
    let s_target = String::from_str("XMAS").expect("XMAS string cannot be created");
    let mut s = String::new();
    let dims= dims_matrix(m.clone());
    let i_max = dims.0;
    let j_max = dims.1;
    if j + s_target.len() > j_max {
        return (false, vec![(0, 0)]);
    };
    if i + s_target.len() > i_max {
        return (false, vec![(0, 0)]);
    };
    let mut indices: Vec<(usize, usize)> = vec![];
    let i_arr: Vec<usize> = (i..i+s_target.len()).collect();
    let j_arr: Vec<usize> = (j..j+s_target.len()).collect();
    for (ii, jj) in i_arr.iter().zip(j_arr.iter()){
            s.push(m.clone()[*ii][*jj]);
            indices.push((*ii, *jj));
    };
    if s.eq(&s_target) {
        return (true, indices);
    } else {
        return (false, indices);
    };
}

fn search_diagonal_up_right(m: Vec<Vec<char>>, i: usize, j: usize) -> (bool, Vec<(usize, usize)>) {
    let s_target = String::from_str("XMAS").expect("XMAS string cannot be created");
    let mut s = String::new();
    let dims= dims_matrix(m.clone());
    let j_max = dims.1;
    if j + s_target.len() > j_max {
        return (false, vec![(0, 0)]);
    };
    if i < s_target.len() -1 {
        return (false, vec![(0, 0)]);
    };
    let mut indices: Vec<(usize, usize)> = vec![];
    let i_arr: Vec<usize> = (i+1-s_target.len()..i+1).rev().collect();
    let j_arr: Vec<usize> = (j..j+s_target.len()).collect();
    for (ii, jj) in i_arr.iter().zip(j_arr.iter()){
            s.push(m.clone()[*ii][*jj]);
            indices.push((*ii, *jj));
    };
    if s.eq(&s_target) {
        return (true, indices);
    } else {
        return (false, indices);
    };
    
}
fn search_diagonal_down_left(m: Vec<Vec<char>>, i: usize, j: usize) -> (bool, Vec<(usize, usize)>) {
    let s_target = String::from_str("XMAS").expect("XMAS string cannot be created");
    let mut s = String::new();
    let dims= dims_matrix(m.clone());
    let i_max = dims.0;
    if j < s_target.len() -1 {
        return (false, vec![(0, 0)]);
    };
    if i + s_target.len() > i_max {
        return (false, vec![(0, 0)]);
    };
    let mut indices: Vec<(usize, usize)> = vec![];
    let i_arr: Vec<usize> = (i..i+s_target.len()).collect();
    let j_arr: Vec<usize> = (j+1-s_target.len()..j+1).rev().collect();
    for (ii, jj) in i_arr.iter().zip(j_arr.iter()){
            s.push(m.clone()[*ii][*jj]);
            indices.push((*ii, *jj));
    };
    if s.eq(&s_target) {
        return (true, indices);
    } else {
        return (false, indices);
    };
}
fn search_diagonal_up_left(m: Vec<Vec<char>>, i: usize, j: usize) -> (bool, Vec<(usize, usize)>) {
    let s_target = String::from_str("XMAS").expect("XMAS string cannot be created");
    let mut s = String::new();
    if j < s_target.len() -1{
        return (false, vec![(0, 0)]);
    };
    if i < s_target.len()-1 {
        return (false, vec![(0, 0)]);
    };
    let mut indices: Vec<(usize, usize)> = vec![];
    let i_arr: Vec<usize> = (i+1-s_target.len()..i+1).rev().collect();
    let j_arr: Vec<usize> = (j+1-s_target.len()..j+1).rev().collect();
    for (ii, jj) in i_arr.iter().zip(j_arr.iter()){
            s.push(m.clone()[*ii][*jj]);
            indices.push((*ii, *jj));
    };
    if s.eq(&s_target) {
        return (true, indices);
    } else {
        return (false, indices);
    };
}


fn search_xmas_in_matrix(m: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut indices: Vec<(usize, usize)> = vec![];
    for (i, line) in m.clone().into_iter().enumerate(){
        for (j, _) in line.into_iter().enumerate(){
            let res = search_horizontal(m.clone(), i, j);
            if res.0 { //res.0 is true if matched
                indices.append(&mut res.1.clone());
            };
            let res = search_horizontal_back(m.clone(), i, j);
            if res.0 { //res.0 is true if matched
                indices.append(&mut res.1.clone());
            };
            let res = search_vertical(m.clone(), i, j);
            if res.0 { //res.0 is true if matched
                indices.append(&mut res.1.clone());
            };
            let res = search_vertical_back(m.clone(), i, j);
            if res.0 { //res.0 is true if matched
                indices.append(&mut res.1.clone());
            };
            let res = search_diagonal_down_left(m.clone(), i, j);
            if res.0 { //res.0 is true if matched
                indices.append(&mut res.1.clone());
            };
            let res = search_diagonal_up_left(m.clone(), i, j);
            if res.0 { //res.0 is true if matched
                indices.append(&mut res.1.clone());
            };
            let res = search_diagonal_down_right(m.clone(), i, j);
            if res.0 { //res.0 is true if matched
                indices.append(&mut res.1.clone());
            };
            let res = search_diagonal_up_right(m.clone(), i, j);
            if res.0 { //res.0 is true if matched
                indices.append(&mut res.1.clone());
            };
        };  
    }
    indices
}

fn check_diagonal_up_left_down_right(m: Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if m[i-1][j-1] == 'S' && m[i+1][j+1] == 'M'{
        true
    } else if m[i-1][j-1] == 'M' && m[i+1][j+1] == 'S' {
        true
    } else {
        false
    }
}

fn check_diagonal_down_left_up_right(m: Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if m[i+1][j-1] == 'S' && m[i-1][j+1] == 'M' && m[i][j] == 'A' {
        true
    } else if m[i+1][j-1] == 'M' && m[i-1][j+1] == 'S' && m[i][j] == 'A' {
        true
    } else {
        false
    }
}

fn is_mas_cross(m: Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 1 {
        false
    } else if i > m.len()-2 {
        false
    } else if j < 1 {
        false
    } else if j > m[0].len()-2 {
        false
    }  else {
        check_diagonal_up_left_down_right(m.clone(), i, j) && check_diagonal_down_left_up_right(m.clone(), i, j)
    }

}

fn search_mas_cross_in_matrix(m: Vec<Vec<char>>) -> (i32, Vec<(usize, usize)>) {
    let mut n = 0;
    let mut indices: Vec<(usize, usize)> = vec![];
    for (i, line) in m.clone().into_iter().enumerate(){
        for (j, _) in line.into_iter().enumerate(){
                if is_mas_cross(m.clone(), i, j) {
                    indices.push((i-1, j-1));
                    indices.push((i-1, j+1));
                    indices.push((i, j));
                    indices.push((i+1, j-1));
                    indices.push((i+1, j+1));
                    n += 1;
                }
            };
        };
    (n, indices)
} 
            
fn day_4() {
    let s = fs::read_to_string("day4.txt").expect("Parsing of file went wrong");
    let m  = from_str_to_matrix(&s);

    // Part 1: find all instances of xmax
    let indices = search_xmas_in_matrix(m.clone());
    // display_xmas_matrix(m.clone(), indices.clone());
    println!("{:}", indices.len() / 4);

    // Part 2: find all mas instances in an x shape
    let res = search_mas_cross_in_matrix(m.clone());
    // display_xmas_matrix(m.clone(), res.1.clone());
    println!("n indices: {:}", res.0);

}

#[derive(Debug)]
#[derive(Clone)]
struct OrderPair {
    n1: u8,
    n2: u8
}

impl OrderPair {
    fn check(&self, order: &Vec<u8>) -> bool {
        let idx_n1 = order.iter().position(|r| r == &self.n1).unwrap();
        let idx_n2 = order.iter().position(|r| r == &self.n2).unwrap();
        return idx_n1 < idx_n2
    }
    fn apply(&self, order: &mut Vec<u8>) {
        let idx_n1 = order.iter().position(|r| r == &self.n1).unwrap();
        let idx_n2 = order.iter().position(|r| r == &self.n2).unwrap();
        if idx_n1 > idx_n2 {
            order.swap(idx_n1, idx_n2);
        }
    }
    fn is_relevant(&self, order: &Vec<u8>) -> bool {
        order.contains(&self.n1) && order.contains(&self.n2)
    }
}


fn parse_rules_and_orders(s: String) -> (Vec<OrderPair>, Vec<Vec<u8>>) {
    let mut pairs: Vec<OrderPair> = vec![];
    let mut orders: Vec<Vec<u8>> = vec![];
    let mut parse_pairs: bool = true;
    for l in s.split("\n"){
        if l == "" {
            parse_pairs = false;
            continue;
        }
        if parse_pairs {
            let numbers: Vec<&str> = l.split("|").collect();
            let q = OrderPair{n1: numbers[0].parse().unwrap(), n2: numbers[1].parse().unwrap()};
            pairs.push(q);
        } else {
            let order_str: Vec<&str> = l.split(",").collect();
            let mut order: Vec<u8> = vec![];
            for n in order_str {
                order.push(n.parse().unwrap());
            };
            orders.push(order);
        };
    };
    (pairs, orders)
}


fn order_is_correct(rules: &Vec<OrderPair>, order: &mut Vec<u8>) -> bool {
    for rule in rules.iter() {
        if order.contains(&rule.n1) && order.contains(&rule.n2) {
            let c  = rule.check(&order);
            if !c {
                return false
            };
        };
    };
    true
}

fn get_middle_page_number(v: &Vec<u8>) -> u8 {
    if v.len() % 2 == 0 { // number of values is even, middle page number cannot be determined. Zero is used to disregard value
        return 0
    } else {
        let idx: f64 = v.len() as f64 / 2.0;
        let idx: usize = idx.floor() as usize;
        return v[idx]
    }
}

fn sort_order_list(order: Vec<u8>, rules: &Vec<OrderPair>) -> Vec<u8> {
    let mut relevant_rules: Vec<OrderPair> = vec![];
    for rule in rules{
        if rule.is_relevant(&order) {
            relevant_rules.push(rule.clone());
        };
    };
    let mut new_order = order.clone();
    while ! order_is_correct(&relevant_rules, &mut new_order) {
        let temp_order = new_order.clone();
        for (n1, n2) in temp_order[..temp_order.len()-1].iter().zip(temp_order[1..].iter()) {
            for rule in relevant_rules.iter(){
                let v: Vec<u8> = [*n1, *n2].to_vec();
                
                if rule.is_relevant(&v){
                    rule.apply(&mut new_order);
                };
            };
        };
        //Naive method swapping 2 elements, seemed to be stuck!
        // let rule = relevant_rules[i%relevant_rules.len()].clone();
        // let new_order: Vec<u8> = rule.apply(&new_order);
        // i += 1;
    };
    new_order
}

fn day_5() {
    let s = fs::read_to_string("day5.txt").expect("Parsing of file went wrong");
    let res = parse_rules_and_orders(s);
    let rules = res.0;
    let orders = res.1;

    //Part 1: get all correct orders and sum middle values
    let mut correct_orders: Vec<Vec<u8>> = vec![];
    let mut incorrect_orders: Vec<Vec<u8>> = vec![];
    let mut total: i64 = 0;
    for order in orders.iter() {
        let mut order = order.clone();
        if order_is_correct(&rules, &mut order) {
            correct_orders.push(order.clone());
            total += get_middle_page_number(&order) as i64;
        } else {
            incorrect_orders.push(order.clone());
        }
    };
    println!("{:?}", total);

    // Part 2: set incorrect ordered numbers into correct order and sum middle values
    let mut sorted_orders: Vec<Vec<u8>> = vec![];
    let mut total:i32 = 0;
    for incorrect_order in incorrect_orders.iter(){
        let sorted = sort_order_list(incorrect_order.clone(), &rules);
        sorted_orders.push(sorted.clone());
        total += get_middle_page_number(&sorted) as i32;
    };
    println!("{:?}", total);


}
