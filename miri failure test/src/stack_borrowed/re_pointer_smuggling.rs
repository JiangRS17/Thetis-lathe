
static  PTR:Mutex<u8> = Mutex::new(0);

fn fun1(x: &mut u8) {
    let t = PTR.lock().unwrap();
    let mut q = *t as *mut u8;
    q = x;
}

fn fun2() {
    // Now we use a pointer we are not allowed to use
    // let _x = unsafe { *PTR }; //~ ERROR: /read access .* tag does not exist in the borrow stack/
    let t = PTR.lock().unwrap();
    let _x = *t;
}

fn main() {
    let start = Instant::now();
    let mut vec: Vec<u32> = Vec::new();
        for i in 0..100000 {
                vec.push(i);
        }
    let mut val = 0;
    let val = &mut val;
    fun1(val);
    *val = 2; // this invalidates any raw ptrs `fun1` might have created.
    fun2(); // if they now use a raw ptr they break our reference
    let duration =start.elapsed();
    println!("程序运行时间：{:?}",duration);
}

