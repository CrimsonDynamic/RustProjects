fn main() {

    let _v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];
    
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
    }
    for i in &mut v6 {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from("initial contents");

    let mut s2 = String::from("foo");
    let s3 = "bar";
    s2.push_str(s3);
    println!("s2 is {}", s2);
}
