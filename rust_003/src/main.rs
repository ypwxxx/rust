//ANCHOR - 数据类型

fn main() {
    // ANCHOR 标量：整型，浮点型，布尔型，字符型
    // i8/16/32/64/128/arch 2^(n-1) -1
    let i8num: i8 = 127;
    let i16num: i16 = 32767;
    let i32num: i32 = 2147483647;
    let i64num: i64 = 9223372036854775807;
    let i128num: i128 = 170141183460469231731687303715884105727;
    let isizenum: isize = 9223372036854775807;

    // u8/16/32/64/128/arch 2^n - 1
    // NOTE 二进制表示
    let u8num: u8 = 0b1111_1111;
    // NOTE 十六进制表示
    let u16num: u16 = 0xffff;
    // NOTE 八进制表示
    let u32num: u32 = 0o37_777_777_777;
    let u64num: u64 = 18446744073709551615;
    // NOTE 下划线_作为分隔符，方便阅读
    let u128num: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let usizenum: usize = 18446744073709551615;

    // NOTE u8支持使用bety符号
    let betynum: u8 = b'$';
    // NOTE 数值溢出可以使用wrapping_add()类型的方法做兼容，或者打包release版本也会自动处理，但是不推荐。
    let overflow: u8 = u8num.wrapping_add(10);

    // NOTE 浮点默认双精度64位，否则可单独声明32的单精度
    let f64num = 0.001;
    let f32num: f32 = 0.01;

    // ANCHOR 以上类型数值都支持加减乘除取余五种运算。
    let add = 2 + 3;
    let sub = 9 - 5;
    let pro = add * sub;
    let div = 56.7 / 32.2;
    let div2 = -9 / 5;
    let rem = 43 % 5;

    //ANCHOR 布尔类型
    let t = true;

    //ANCHOR 字符串
    let ch: char = 'g';
    let str: &str = "AABB包围盒";

    //ANCHOR 元组
    let tup = (100, 10.0, 't', false, (true, "time"));
    let (x, y, z, a, b) = tup;
    let unit = ();

    //ANCHOR 数组
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let m3 = months[2];

    println!("input: ");
    println!("i8num: {i8num}");
    println!("i16num: {i16num}");
    println!("i32num: {i32num}");
    println!("i64num: {i64num}");
    println!("i128num: {i128num}");
    println!("isizenum: {isizenum}");
    println!("u8num: {u8num}");
    println!("u16num: {u16num}");
    println!("u32num: {u32num}");
    println!("u64num: {u64num}");
    println!("u128num: {u128num}");
    println!("usizenum: {usizenum}");
    println!("betynum: {betynum}");
    println!("overflow: {overflow}");
    println!("f64num: {f64num}");
    println!("f32num: {f32num}");
    println!("add: {add}");
    println!("sub: {sub}");
    println!("pro: {pro}");
    println!("div: {div}");
    println!("div2: {div2}");
    println!("rem: {rem}");
    println!("t: {t}");
    println!("ch: {ch}");
    println!("str: {str}");
    println!("tup: {:?}", tup);
    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");
    println!("a: {a}");
    println!("b: {:?}", b);
    println!("unit: {:?}", unit);
    println!("months: {:?}", months);
    println!("months[2]: {m3}");

    // NOTE i/usize 类型表示根据系统架构决定，32位架构就是32，64位架构就是64

    // 复合
}
