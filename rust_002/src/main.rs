//ANCHOR - 变量与可变性

fn main() {
    let x = 5;

    println!("The value of x is: {x}");

    // NOTE tip x = 6; 不可变量再次赋值，导致编译报错

    let x = x + 1;
    // NOTE - x=6 第二个声明隐藏了第一个声明，此时后面再使用x，值为6。

    println!("next value of x is: {x}");

    {
        let x = "a";
        // NOTE - x = 8 第三个声明隐藏了第二个声明，此时后面再使用x，值为8。
        // NOTE - 该x仅存在于本作用域内。

        println!("next value of x is: {x}");
    }

    println!("The value of x is: {x}");
    // NOTE - x的声明回到第二次。

    let mut y = 8;
    // NOTE - 声明可变变量，后续可继续修改变量的值，但是不可更改变量。

    println!("The value of y is: {y}");

    y = 10;
    println!("The value of y is: {y}");

    // NOTE - 隐藏与mut形式根本区别在于，隐藏是真的重新申请了一个变量，而mut是复用。故而其一个类型可变，一个不可。

    // NOTE const TIME_A = 60*60; const常量不能是运行时获取的，只能是提前确定的值。
}
