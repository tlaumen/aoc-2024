
fn main() {
    day_1_1();
}

fn day_1_1() {
    let rdr = csv::Reader::from_path("day1_1.csv");
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
                println!("{}", part);
                if i == 0 {
                    v1.push(part.parse().expect("Not a integer"));
                } else {
                    v2.push(part.parse().expect("Not a integer"));
                }
                i += 1;
            };

        };
    };
    v1.sort();
    v2.sort();
    let mut s: i32 = 0;
    for (l1, l2) in v1.iter().zip(v2.iter()) {
        let diff: i32 = l2 - l1;
        s += diff.abs();
    };
    println!("{}", s);
}
