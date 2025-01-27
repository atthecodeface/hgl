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
    // Simply record the time taken (apparenlty) by Timer::delta()
    //
    // Outputs the distribuion of delays

    let mut record = vec![0u64; 100_000];
    let mut t0 = Timer::default();
    t0.entry();
    for i in 0..record.len() {
        record[i] = t0.delta();
    }

    let mut deltas: HashMap<u64, usize> = HashMap::new();

    for i in 1..record.len() {
        let d = record[i] - record[i - 1];
        *deltas.entry(d).or_default() += 1;
    }

    let mut d: Vec<u64> = deltas.keys().copied().collect();
    d.sort();
    eprintln!("Complete delta distribution");
    for d in &d {
        eprintln!("{d} {}", deltas[d]);
    }

    eprintln!("Percentile distribution");
    let mut p = 0;
    let mut acc = 0;
    let mut sum_to_95 = 0;
    let mut num_to_95 = 0;
    let n = record.len();
    for d in &d {
        acc += deltas[d];
        let acc_p = (acc * 100) / n;
        if acc_p > p {
            p = acc_p;
            eprintln!("{acc_p}, {d}");
        }
        if acc_p < 95 {
            sum_to_95 += (*d as usize) * deltas[d];
            num_to_95 += deltas[d];
        }
    }
    eprintln!("100, {}", d.last().unwrap());
    eprintln!("");
    eprintln!("average of up to 95 {}", sum_to_95 / num_to_95);

    assert!(false);
}
