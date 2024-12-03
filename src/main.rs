use std::{fs, str::FromStr, vec};
use regex::Regex;

fn main() {
    day_3();
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
