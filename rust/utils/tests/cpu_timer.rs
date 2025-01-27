use std::collections::HashMap;

use hgl_utils::cpu_timer::{AccTimer, Timer};

#[test]
fn test_timer() {
    let mut t0 = Timer::default();
    let mut t1 = Timer::default();
    t0.entry();
    t1.entry();
    t0.exit();
    t1.exit();

    let diff = (t0.value() as i64 - t1.value() as i64).abs();
    dbg!(&t0, &t1);
    assert!(
        diff < 10,
        "Expecting the difference to be not a lot; this might fail very infrequently"
    );
}

#[test]
fn test_timer_values() {
    let mut record = vec![0u64; 1000];
    let mut t0 = Timer::default();
    t0.entry();
    for i in 0..1000 {
        record[i] = t0.delta();
    }

    let mut deltas: HashMap<u64, usize> = HashMap::new();

    for i in 1..record.len() {
        let d = record[i] - record[i - 1];
        *deltas.entry(d).or_default() += 1;
    }

    dbg!(&deltas);
    assert!(false);
}
