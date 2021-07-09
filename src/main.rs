#![feature(bench_black_box)]


use core::cmp::Ordering;

fn is_sorted_by<I,F>(arr:&[I], mut compare: F) -> bool
    where
        F: FnMut(&I, &I) -> Option<Ordering>,
{
    arr.windows(2).all(|w|compare(&w[1],&w[0]).unwrap()!=Ordering::Less)
}

fn is_sorted<T>(data: &[T]) -> bool
where
    T: PartialOrd,
{
    is_sorted_by(data, |a,b|a.partial_cmp(b))
}


fn is_sorted2<T>(data: &mut [T]) -> bool
where
    T: PartialOrd,
{

    use is_sorted::IsSorted;
    IsSorted::is_sorted(&mut data.iter())
}


use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    dbg!(is_sorted(&[0,1,2]));
    dbg!(is_sorted_by(&[0,1,2]),|a,b|);

    let mut vec: Vec<u32> = (0..10000000).collect();
    
    //vec.shuffle(&mut thread_rng());
    let mut vec:Vec<_> = vec.into_iter().map(|x|(x,[0u64;10])).collect();

    let v=util::bench_closure(move ||{dbg!(is_sorted(&mut vec));});

    println!("{:?}", v);
}







mod util{
    use std::time::*;
    use core::hint::*;

    fn into_secs(elapsed: std::time::Duration) -> f64 {
        (elapsed.as_secs() as f64) + (f64::from(elapsed.subsec_nanos()) / 1_000_000_000.0)
    }

    pub fn bench_closure(func: impl FnOnce()) -> f64 {
        black_box(bench_closure_ret(func).1)
    }

    pub fn bench_closure_ret<T>(func: impl FnOnce() -> T) -> (T, f64) {
        let instant = Instant::now();
        let a = black_box(func());
        let j = instant_to_sec(instant.elapsed());
        (a, j)
    }

    pub fn instant_to_sec(elapsed: Duration) -> f64 {
        let secs: f64 = elapsed.as_secs() as f64;
        let nano: f64 = elapsed.subsec_nanos() as f64;
        secs + nano / 1_000_000_000.0
    }
}
