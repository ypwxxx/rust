//ANCHOR 控制流

fn func_if(x: i32) {
    let mut ret: &str;

    // NOTE if判断需要表达式返回一个bool值，否则报错
    // NOTE rust这里没有自动类型转换，必须自己手动做好类型确认。
    if x > 20 {
        ret = "> 20";
    } else if x > 10 {
        ret = "> 10";
    } else {
        ret = "<= 10";
    }

    ret = if x == 15 {
        "surperise! you guess right!"
    } else {
        ret
    };

    println!("func if {x} {ret}");
}

fn func_loop() {
    let mut count = 0;
    let ret = loop {
        println!("循环开始");
        count += 1;

        if count == 2 {
            println!("count==2 使用continue跳过本次循环，执行下面的循环。");
            continue;
        }

        if count == 4 {
            println!("count==4 break结束整个循环。");
            break count;
        }

        println!("循环{count}次");
    };

    // NOTE loop也是一个表达式，使用break可以把结果返回出来。
    // NOTE 只是普通循环逻辑，也无需专门返回某个值出来。
    // NOTE 注意continue是不会返回值的。
    // NOTE 注意多个阶段的返回值的类型需要一致。

    println!("loop 演示结束！结果: {ret}");
}

fn func_loop2() {
    let mut count = 0;

    // 设定loop的标签
    let ret = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            println!("loop test {remaining}");

            remaining = loop {
                println!("loop test 111 {remaining}");
                if remaining == 8 {
                    // NOTE 默认跳帧本循环
                    break remaining;
                } else {
                    remaining -= 1;
                    break remaining;
                }
            };

            // if remaining == 8 {
            //     println!("loop test 333");
            //     // NOTE 这里跳出本循环
            //     break;
            // }

            if count == 5 {
                // NOTE 跳出指定标签的循环
                // NOTE 一样可以指定返回值
                break 'counting_up "ok?";
            } else {
                count += 1;
            }
            remaining -= 1;
        }

        // NOTE 一个loop内 仅能跳出该循环和设定了标签的父循环
        // NOTE 存在同级多个循环时，不能互相跳出，仅能操作自己循环或者父循环。
        // 'loop3: loop {
        //     if remaining == 8 {
        //         break 'loop2;
        //     }
        // }

        count += 1;

        if count == 10 {
            break "default";
        };
    };
    println!("End count = {count}");
    println!("End ret = {ret}");
}

fn func_while(mut count: i32) {
    while count > 0 {
        println!("while 循环{count}次");
        count -= 1;
    }
}

fn func_my_while(mut count: i32) {
    loop {
        if count <= 0 {
            break;
        }
        println!("my while 循环{count}次");
        count -= 1;
    }
}

fn func_for(list: &[i32]) {
    for ele in list {
        println!("value is {ele}");
    }
}

fn func_while2(list: &[i32]) {
    let len = list.len();
    let mut i = 0;

    while i < len {
        println!("list[{i}] = {:?}", list[i]);
        i += 1;
    }
}

fn main() {
    println!("Hello, rust_006!");

    func_if(15);
    func_loop();
    func_loop2();
    func_while(3);
    func_my_while(2);
    func_while2(&[2, 3, 51, 5, 6, 7, 8]);
    func_for(&[0, 1, 3, 4, 5, 6, 0]);
}
