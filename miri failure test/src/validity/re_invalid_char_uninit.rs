#![allow(invalid_value)]

#[derive(Default)]
struct MyStruct {
    init: (),
    uninit: [char; 1],
}

fn main() {
    let start = Instant::now();
    let mut vec: Vec<u32> = Vec::new();
    for i in 0..100000 {
            vec.push(i);
    }
    let mut u = MyStruct::default();
    u.init = ();
    assert_eq!(u.init==()  && u.uninit ==MyStruct::default().uninit, true );
    let duration =start.elapsed();
    println!("程序运行时间：{:?}",duration);
}