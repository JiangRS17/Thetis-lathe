// static mut LEAK: usize = 0;
static LEAK: Mutex<usize> = Mutex::new(0);

fn fill(v: &mut i32) {
        // LEAK = v as *mut _ as usize;
        let t = v as *mut _ as usize;
        let mut q = LEAK.lock().unwrap();
        *q = t;
}

fn evil() {
    // let _ = unsafe { 
    //     &mut *(LEAK as *mut i32) 
    // }; //~ ERROR: is a dangling pointer
    // let t = LEAK as *mut i32;
    let mut t = LEAK.lock().unwrap();
    let q = *t as *mut i32;
    assert!(!q.is_null()); // 添加断言，判断 q 是否为空
    let _ = unsafe {
        &mut *q
    };
}

fn main() {
    let _y;
    {
        let mut x = 0i32;
        fill(&mut x);
        _y = x;
    }
    // Now we use a pointer to `x` which is no longer in scope, and thus dead (even though the
    // `main` stack frame still exists). We even try going through a `usize` for extra sneakiness!
    evil();
}