// 包与crate
mod garden;
mod garden2;
use crate::garden2::veg;

fn main() {
    println!("Hello, rust_010!");

    veg::getTest();

    //NOTE - crate是最小编译单元
    //NOTE - crate有二进制和库两种形式，其中库类似lib，二进制类比可执行程序。
    //NOTE - 二进制crate必须要有一个main函数作为入口，库不需要。
    //NOTE - 一个包有一或者多个crate，会有一个Cargo.toml阐述这些crate。
    //NOTE - 包可以有多个二进制crate，但是最多只有一个库crate。
    //NOTE - 包最少也有一个crate
    //NOTE - 默认 src/main.rs 是一个与包同名的二进制crate。
    //NOTE - 默认 src/lib.rs 是一个与包同名的库crate。
    //NOTE - src/main.rs 与 src/lib.rs可以同时存在，同时编译。
    //NOTE - src/bin 下存放一个个二进制crate。

    //NOTE - pub的作用，可以作用的位置，产生的效果
    //NOTE - super的意义。
    //NOTE - 使用as为模块取别名
    //NOTE - 使用use std::{cmp::Ordering, io};减少从相同库中拿取项的代码
    //NOTE - glob运算符：*。可以把库里的所有相引入，但是自己要知道自己在做什么。
}
