use std::collections::HashMap;

use hgl_utils::cpu_timer::{AccTimer, Timer};

fn abs_diff(a: u64, b: u64) -> u64 {
    ((a as i64) - (b as i64)).abs() as u64
}

#[test]
fn test_timer() {
    let mut t0 = Timer::default();
    let mut t1 = Timer::default();
    t0.entry();
    t1.entry();
    t0.exit();
    t1.exit();

    let diff = abs_diff(t0.value(), t1.value());
    dbg!(&t0, &t1);
    assert!(
        diff < 100,
        "Expecting the difference to be not a lot; this might fail very infrequently"
    );

    t0.clear();
    assert_eq!(t0.value(), 0, "Value should be 0 at clear");
}

fn do_work() -> () {
    let mut t = Timer::default();
    for _ in 0..100 {
        t.entry();
        t.exit();
    }
}

#[test]
fn test_acc_timer() {
    const N: usize = 10_000;
    let mut t0 = AccTimer::default();
    let mut passed = false;

    let mut acc_10x = 0;
    for _retries in 0..10 {
        let t_acc_x10 = {
            for _ in 0..N * 10 {
                t0.entry();
                do_work();
                t0.exit();
            }
            t0.acc()
        };
        t0.clear();
        let t_acc_10x = {
            let mut acc = 0;
            for _ in 0..10 {
                for _ in 0..N {
                    t0.entry();
                    do_work();
                    t0.exit();
                }
                acc += t0.acc();
                t0.clear();
            }
            acc
        };

        assert!(t_acc_10x != 0, "10*N iterations cannot be zero");
        let diff = abs_diff(t_acc_x10, t_acc_10x);
        dbg!(diff, t_acc_x10, t_acc_10x);
        if diff < t_acc_10x * 1 / 10 {
            passed = true;
            acc_10x = t_acc_10x;
            break;
        }
    }
    assert!(
        passed,
        "Expected diff to be within 10% in at least one attempt"
    );

    let mut zeros = 0;
    let mut outliers = 0;
    for _ in 0..N {
        t0.clear();
        t0.entry();
        do_work();
        t0.exit();
        let v = t0.value() as usize;
        if v == 0 {
            zeros += 1;
        }
        if v * 10 * N < acc_10x as usize * 80 / 100 {
            outliers += 1;
        }
        if v * 10 * N > acc_10x as usize * 2 {
            outliers += 1;
        }
        dbg!(v);
    }

    dbg!(acc_10x, zeros, outliers, (acc_10x as usize) / 10 / N);

    assert!(zeros != N, "Cannot all be zero");
    assert!(outliers < N * 1 / 10, "Fewer than 10% should be outliers");
    assert!(false);
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
}
