//ANCHOR slice引用

fn first_word(str: &String) -> &str {
    let byte = str.as_bytes().iter().enumerate();

    for (i, &value) in byte {
        if value == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}

fn main() {
    let t = String::from("hel lo");

    let mut t2 = "world";
    println!("t2: {}", t2);

    t2 = "xxxx";

    // let t2 = &t[0..1];

    // let slice = &t[0..2];
    // let slice = &t[..2];

    // let len = t.len();

    // let slice = &t[3..len];
    // let slice = &t[3..];

    // let len = t.len();

    // let slice = &t[0..len];
    // let slice = &t[..];

    println!("test: {}", first_word(&t));
    println!("test2: {}", t2);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    //NOTE - 数组也有slice引用
}
