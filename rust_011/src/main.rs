//ANCHOR - 常用集合
use std::{collections::HashMap, ops::Mul};

fn main() {
    println!("Hello, rust011!");

    let arr = ['c'; 10];
    let mut v = vec![1, 2, 3];

    v.push(10);

    println!("arr = {:?}", arr);
    println!("arr = {:?}", v);

    let i2 = v[2];
    let ii2 = v.get(2);

    println!("The third element is {i2}");

    match ii2 {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //NOTE - String
    let mut s1 = "go rust".to_string();
    let s2 = "uuu";

    println!("The s1 element is {s1}");

    s1.push_str(" foo");

    println!("The s1 element is {s1}");

    s1.push('R');

    println!("The s1 element is {s1}");

    // s1 = s1 + s2;

    // println!("The s1 element is {s1}");

    let s3 = format!("{s1}-{s2}");

    println!("The s3 element is {s3}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("The s element is {s}");

    //NOTE - 注意字节，标量，字形簇的区别
    let s4 = "नमस्ते".to_string();

    for c in s4.chars() {
        println!("{c}");
    }

    //NOTE - HashMap
    let mut obj = HashMap::new();

    obj.insert('k', 20);

    let n = obj.get(&'k').copied().unwrap_or(0);

    //TODO - 完成最后的小目标
    let mut list: Vec<i32> = vec![-1, -3, 8, 9, 5, -5, 9, 0, 9, 7, 12, 32, 543, 23];
    check_vec(&mut list);
}

fn check_vec(list: &mut Vec<i32>) {
    let mut info: HashMap<i32, u32> = HashMap::new();
    let mut t = 0;

    list.sort();
    // let t = list.len() as u32 / 2;

    for i in list {
        println!("i = {i}");
        let n = info.entry(*i).or_insert(0);
        *n += 1;
    }

    println!("info: {:?}", info);

    for (k, v) in info {
        if v > t {
            println!("{k}-{v}");
            t = v;
        }
    }
}
