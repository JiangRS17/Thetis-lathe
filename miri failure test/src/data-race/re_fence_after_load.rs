// We want to control preemption here. Stacked borrows interferes by having its own accesses.
//@compile-flags: -Zmiri-preemption-rate=0 -Zmiri-disable-stacked-borrows
// use std::sync::atomic::{fence, AtomicUsize, Ordering};
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

fn main() {
    let start = Instant::now();
    // static mut V: u32 = 0;
    static V:Mutex<u32> = Mutex::new(0);
    let a = Arc::new(AtomicUsize::default());
    let b = a.clone();
    thread::spawn(move || {
        // unsafe { V = 1 }
        let mut t = V.lock().unwrap();
        *t  = 1 ;
        b.store(1, Ordering::SeqCst);
    });
    thread::sleep(Duration::from_millis(1));
    fence(Ordering::SeqCst);
    // Imagine the other thread's actions happening here.
    // assert_eq!(a.load(Ordering::Relaxed), 1);
    let mut t2 = V.lock().unwrap();
    *t2 = 2;
    // unsafe { V = 2 } //~ERROR: Data race detected between (1) non-atomic write on thread `<unnamed>` and (2) non-atomic write on thread `main`
    let duration =start.elapsed();
    println!("程序运行时间：{:?}",duration);
}