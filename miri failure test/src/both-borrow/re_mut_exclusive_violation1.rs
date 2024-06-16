//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows

fn demo_mut_advanced_unique(our: &mut i32) -> i32 {
    unknown_code_1(&*our);

    // This "re-asserts" uniqueness of the reference: After writing, we know
    // our tag is at the top of the stack.
    *our = 5;

    unknown_code_2();

    // We know this will return 5
    *our
}

// Now comes the evil context
use std::ptr;
use std::sync::Arc;

static  LEAK: Arc<Mutex<*mut i32>> = Arc::new(Mutex::new(ptr::null_mut()));
lazy_static! {
    static  ref LEAK: Mutex<*mut i32> = Mutex::new(ptr::null_mut());
}

fn unknown_code_1(x: &i32) {
    let t = LEAK.lock().unwrap;
    *t = x as *mut i32;
    // unsafe {
    //     LEAK = x as *const _ as *mut _;
    // }
}



// fn unknown_code_2() {
//     unsafe {
//         *LEAK.lock().unwrap() = 7;
//     }
// }


fn unknown_code_2() {
    let t = LEAK.lock().unwrap;
    *t = 7;
    // unsafe {
    //     *LEAK = 7;
    //     //~[stack]^ ERROR: /write access .* tag does not exist in the borrow stack/
    //     //~[tree]| ERROR: /write access through .* is forbidden/
    // }
}

fn main() {
    demo_mut_advanced_unique(&mut 0);
}
