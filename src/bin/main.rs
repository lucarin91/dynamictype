use dynamictype::DType;

fn main() {
    let a: DType = 10.into();
    let b: DType = 3.into();
    let c = a.clone() + b.clone();
    println!("{} + {} = {}", a, b, c);

    let a: DType = 10.into();
    let b: DType = true.into();
    let c = a.clone() + b.clone();
    println!("{} + {} = {}", a, b, c);

    let a: DType = "hello".into();
    let b: DType = true.into();
    let c = a.clone() + b.clone();
    println!("{} + {} = {}", a, b, c);

    let a: DType = "1".into();
    let b: DType = 1.into();
    let c = a.clone() + b.clone();
    println!("{} + {} = {}", a, b, c);

    let a: DType = 1.into();
    let b: DType = "1".into();
    let c = a.clone() + b.clone();
    println!("{} + {} = {}", a, b, c);

    let a: DType = "asd".into();
    let b: DType = 1.into();
    let c = a.clone() + b.clone();
    println!("{} + {} = {}", a, b, c);

    let a: DType = 1.into();
    let b: DType = "asd".into();
    let c = a.clone() + b.clone();
    println!("{} + {} = {}", a, b, c);
}
