use std::thread;
use std::sync::{Mutex, Arc};
fn main() {
        println!("적분 결과 : {}",get_frac_val_multi(0.0, 1.0, 100));
}
fn get_frac_val(a : f64, b : f64, inf : i64) -> f64{
    let delta : f64 = (b - a) / (inf as f64);
    let mut sum : f64 = 0.0;
    for i in 0..inf {
       sum += 0.5 *(target_func(a + delta * (i as f64)) + target_func(a + delta * ((i + 1) as f64))) * delta;
    }
    return sum;
}
fn target_func(x : f64)-> f64{
    return x;
}
fn get_frac_val_multi(a : f64, b : f64, inf : i64) -> f64{
   
    let delta : f64 = (b - a) / 10.0;
    let counter = Arc::new(Mutex::new(0.0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += get_frac_val(a + (delta * i as f64), a + (delta * (i + 1) as f64), inf);
            println!("{}번 스레드 : 폐구간[{},{}] 적분 완료", i, a + (delta * i as f64), a + (delta * (i + 1) as f64));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let sum : f64 = *counter.lock().unwrap();
   return sum;
}