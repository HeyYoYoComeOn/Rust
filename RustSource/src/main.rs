fn main() {
    let a = 12;
    println!("Hello, world!");
    println!("The number is {}",a);
    println!("The first number is {},another number is {}",a,a);

    const b: i32= 456;
    let b  = 789;
    println!("The number is {}",b);

    let _c: u64 = 128; //这里声明了 c 为无符号 64 位整型变量，如果没有声明类型，a 将自动被判断为有符号 32 位整型变量，这对于 a 的取值范围有很大的影响。

}
