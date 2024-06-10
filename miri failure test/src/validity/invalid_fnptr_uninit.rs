#![allow(invalid_value)]

union MyUninit {
    init: (),
    uninit: [fn(); 1],
}


fn main() {
    let start = Instant::now();
        let mut vec: Vec<u32> = Vec::new();
        for i in 0..100000 {
                vec.push(i);
        }
    let _b = unsafe { MyUninit { init: () }.uninit};  //~ ERROR: constructing invalid value

    let duration =start.elapsed();
    println!("程序运行时间：{:?}",duration);
}

