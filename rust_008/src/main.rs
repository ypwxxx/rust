//ANCHOR 结构体

// struct User {
//     name: String,
//     email: String,
//     age: u8,
//     tag: u32,
// }

// struct Vector2(i32, i32);
// struct Vector3(i32, i32, i32);

// struct Test;

// NOTE 计算面积1
fn get_area_1(width: u32, height: u32) -> u32 {
    width * height
}

// NOTE 计算面积2
fn get_area_2(size: (u32, u32)) -> u32 {
    size.0 * size.1
}

// NOTE 计算面积3
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    // 方法
    fn area(self) -> u32 {
        self.width * self.height
    }

    // 关联函数
    fn sql(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn get_area_3(rect: &Rect) -> u32 {
    rect.width * rect.height
}

fn main() {
    println!("Hello, rust_008!");

    let w = 32;
    let h = 10;
    let rect = Rect {
        width: 12,
        height: 12,
    };

    println!("w: {w}, h: {h}, rect: {:#?}", rect);

    println!("calc rect 1: {}", get_area_1(w, h));
    println!("calc rect 2: {}", get_area_2((w, h)));
    println!("calc rect 3: {}", get_area_3(&rect));
    println!("calc rect 4: {}", rect.area());
    println!("calc rect 5: {:?}", Rect::sql(10));

    // let xiaoming = User {
    //     name: String::from("小明"),
    //     email: String::from("xxx@123.com"),
    //     age: 19,
    //     tag: 100,
    // };

    // println!("user1.name: {:?}", xiaoming.name);
    // println!("user1.email: {:?}", xiaoming.email);
    // println!("user1.age: {:?}", xiaoming.age);
    // println!("user1.tag: {:?}", xiaoming.tag);

    // let user2 = User {
    //     name: String::from("小刚"),
    //     ..xiaoming
    // };

    // println!("user2.name: {:?}", user2.name);
    // println!("user2.email: {:?}", user2.email);
    // println!("user2.age: {:?}", user2.age);
    // println!("user2.tag: {:?}", user2.tag);

    // let v2 = Vector2(0, 0);
    // let v3 = Vector3(1, 2, 0);

    // println!("v2: {:?}", v2.0);
    // println!("v2: {:?}", v3.1);

    // let t = Test;
}
