//ANCHOR 函数

fn test1() -> i32 {
    10
}

// NOTE 指定参数类型，指定返回类型。
fn test2(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");

    let t1 = test1();
    let t2 = test2(t1);
    // NOTE 大括号内最后一行是一个不带分号的表达式，可以输出一个带类型的值给语句。
    let t3 = {
        let n = 9;
        n + t2
        //ANCHOR n + t2; 这个带分号会导致错误，因为就变成一个语句了。
    };

    println!("function test1: {t1}");
    println!("function test2: {t2}");
    println!("function t3: {t3}");
}

//NOTE - 1. 函数fn定义
//NOTE - 2. 小括号后面接大括号
//NOTE - 3. 有参数要说明个数与类型
//NOTE - 4. 有返回值要说明返回值类型
//NOTE - 5. rust是基于表达式的语言，其中的语句和表达式是完全不一样的。
//NOTE - 6. 语句没有返回值，表达式有。
//NOTE - 7. 表达式后面加了一个分号，就会变成语句，就不会返回值，会导致一些需要返回值的表达式错误。
//NOTE - 8. let i = 6;是一个语句，6是表达式，返回6这个值给let声明的变量。
