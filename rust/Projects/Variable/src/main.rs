fn main() {
    // 不可变变量 let
    let a1:i32 = 10;
    // 可变变量 let mut
    let mut a2:i32 = 20;
    a2 = 30;

    // 常量 const
    // 常量必须显示数据类型
    const b1:i32 = 40;

    // 变量隐藏
    // 变量名可以相同，10内存空间没有消失，而是被隐藏起来
    let a = 10;
    let a = 'c';

    // 数据类型： 标量 + 复合
    // 标量：整型、浮点型、布尔型、字符型
    // 整型
    let i: i8;
    let i: i16;
    let i: i32;
    let i: i64;
    let i: i128;
    let i: isize;

    let u: u8;
    let u: u16;
    let u: u32;
    let u: u64;
    let u: u128;
    let u: usize;
    // 浮点型
    let f:f32;
    let f:f64;
    //布尔型
    let b:bool = true;
    let b:bool = false;
    // 字符型
    let c:char = 'A';

    // 复合类型：元组 + 数组
    // 元组
    // 数据类型可不同，一旦申明 ，长度不可变
    // 单元元组，（），代表空值
    let tup = (100, 1.0, 'A', true);
    println!("{} {} {} {}", tup.0, tup.1, tup.2, tup.3);
    // 数组
    // 数据类型必须相同，一旦申明 ，长度不可变
    let arr1 = [1, 2, 3, 4];
    println!("{} {} {} {}", arr[0], arr[1], arr[2], arr[3]);
    // 类型；元素数量
    let arr2: [i32; 8];
    // 赋值5个0
    let arr3 = [0;5];
}
