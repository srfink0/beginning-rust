/* After some time of computation,
it prints two fractional numbers.*/
fn main() {
    use std::time::Instant;
    fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
        let t = t2 - t1;
        t.as_secs() as f64 * 1000.
            + t.subsec_nanos() as f64 / 1e6
    }

    const SIZE: usize = 40_000;
    let t0 = Instant::now();
    let mut v = Vec::<usize>::new();
    for i in 0..SIZE {
        v.insert(0, i);
        v.insert(0, SIZE + i);
        v.pop();
        v.insert(0, SIZE * 2 + i);
        v.pop();
    }
    let t1 = Instant::now();
    while v.len() > 0 {
        v.pop();
    }
    let t2 = Instant::now();
    print!("{} {}", elapsed_ms(t0, t1), elapsed_ms(t1, t2));
}
