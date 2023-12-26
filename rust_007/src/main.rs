//ANCHOR 所有权

//NOTE 定义1: 每一个值只有一个所有者
//NOTE 定义2: 任一时刻有且只有一个所有者
//NOTE 定义3: 所有者离开作用域时，值会被丢弃
//NOTE 定义4: rust永远不会自动创建数据的“深拷贝”，也就是不会自动复制堆上的数据。
//NOTE 基础类型，或者基础衍生的可确认空间的类型，都支持Copy特性。
//NOTE 存放于堆上的数据，重新赋值时，原始被赋值变量就会被销毁，不可用。

fn owner_doc() {
    let s: String = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...

    // s.push_str("1"); // ... 所以到这里不再有效, 使用s会导致编译报错

    let x: i32 = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
    println!("x = {x}"); // 所以在后面可继续使用 x
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

//NOTE 有时候我们只是需要使用一个数据，但并不需要获得所有权
//NOTE 这时可以使用引用(&)，来借用我们希望的数据
//NOTE 借用不是拥有，只能使用这个数据，不能改变。
fn reference_str(str: &String) -> usize {
    println!("借用来的str: {str}");
    //NOTE str.push_str("11"); 这里不能向数据内写入，因为没有所有权。
    str.len()
}

//NOTE 为了满足借用同时还能改变数据的需求，这里使用了可变引用
//NOTE 可变引用也有限制，同一生命周期内，仅能有一个。
fn reference_change(str: &mut String) {
    println!("可变引用: {str}");
    str.push_str("change !!");
    println!("可变引用: {str}");
}

//NOTE 这里单独说明一下可变引用不能在同一时间多次使用的问题。
//NOTE rust为了避免产生“数据竞争”的情况，在编译阶段就拦截了此类错误。
//NOTE 数据竞争：
//NOTE - 两个或更多指针同时访问同一数据。
//NOTE - 至少有一个指针被用来写入数据。
//NOTE - 没有同步数据访问的机制。

fn main() {
    owner_doc();

    let heo = String::from("hello hero.");
    let mut heo2 = String::from("hero 2 ");

    let heo_1: usize = reference_str(&heo);
    let heo_2: usize = reference_str(&heo);
    reference_change(&mut heo2);

    let her_4: &mut String = &mut heo2; //NOTE 这里可以借用，因为上一个借用已经在函数作用域结束时，还回了借用。
                                        //NOTE let her_5: &mut String = &mut heo2; 不同在同一时间（作用域内），多次借用heo2。

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题
    // println!("test: {r1} {r2} {r3}");
    //NOTE 同一个变量的不可变引用也不能和可变引用同时存在
    //NOTE 存在一个可变引用时，不能再存在其他可变或不可变引用。
    //NOTE 这里也存在一个例外，如果r3的使用在r1,r2使用完了之后，就可以正常声明了。

    println!("heo 长度: {}", heo_1);
    println!("heo2 长度: {}", heo_2);
    println!("her_4 {}", her_4);
}

//NOTE 引用总结
// - 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// - 引用必须总是有效的。
