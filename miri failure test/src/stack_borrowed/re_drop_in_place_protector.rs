//! Test that drop_in_place retags the entire place,
//! invalidating all aliases to it.

// A zero-sized drop type -- the retagging of `fn drop` itself won't
// do anything (since it is zero-sized); we are entirely relying on the retagging
// in `drop_in_place` here.
use std::ptr;
use std::sync::Arc;


#[repr(transparent)]
struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {
        let t = P.lock().unwrap();
        let _val = *t;
        // unsafe {
        //     let _val = *P;
        //     //~^ ERROR: /not granting access .* because that would remove .* which is strongly protected/
        // }
    }
}

// static mut P: *mut u8 = core::ptr::null_mut();
static P: Arc<Mutex<*mut u8>> = Arc::new(Mutex::new(ptr::null_mut()));
lazy_static! {
    static  ref P: Mutex<*mut u8> = Mutex::new(ptr::null_mut());
}

fn main() {
    unsafe {
        let mut x = (HasDrop, 0u8);
        let x = core::ptr::addr_of_mut!(x);
        P = x.cast();
        core::ptr::drop_in_place(x);
    }
}
