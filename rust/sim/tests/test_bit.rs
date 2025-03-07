use hgl_sim::prelude::sim::*;

#[test]
fn format() -> Result<(), String> {
    assert_eq!(
        &SimFormatValue::value_string(&Bit::F, fmt::AS_HEX | fmt::AS_BIN),
        "0"
    );
    assert_eq!(
        &SimFormatValue::value_string(&Bit::T, fmt::AS_HEX | fmt::AS_BIN),
        "1"
    );
    assert_eq!(
        &SimFormatValue::value_string(&Bit::F, fmt::AS_HEX | fmt::AS_BIN | fmt::HDR),
        "1b0"
    );
    assert_eq!(
        &SimFormatValue::value_string(&Bit::T, fmt::AS_HEX | fmt::AS_BIN | fmt::HDR),
        "1b1"
    );

    assert_eq!(
        &SimFormatValue::value_string(&false, fmt::AS_HEX | fmt::AS_BIN),
        "0"
    );
    assert_eq!(
        &SimFormatValue::value_string(&true, fmt::AS_HEX | fmt::AS_BIN),
        "1"
    );
    assert_eq!(
        &SimFormatValue::value_string(&false, fmt::AS_HEX | fmt::AS_BIN | fmt::HDR),
        "1b0"
    );
    assert_eq!(
        &SimFormatValue::value_string(&true, fmt::AS_HEX | fmt::AS_BIN | fmt::HDR),
        "1b1"
    );
    Ok(())
}

#[test]
fn construct() -> Result<(), String> {
    let f0 = Bit::F;
    let f1 = Bit::default();
    let t0 = !f0;
    let t1 = t0;

    assert!(f0.is_false(), "F is false");
    assert!(t0.is_true(), "T is false");

    assert!((f0 & t0).is_false(), "F & T is false");
    assert!((t0 & f0).is_false(), "T & F is false");
    assert!((f1 & f0).is_false(), "F & F is false");
    assert!((t1 & t0).is_true(), "T & T is true");

    assert!((f0 | t0).is_true(), "F | T is true");
    assert!((t0 | f0).is_true(), "T | F is true");
    assert!((f1 | f0).is_false(), "F | F is false");
    assert!((t1 | t0).is_true(), "T | T is true");

    assert!((f0 ^ t0).is_true(), "F ^ T is true");
    assert!((t0 ^ f0).is_true(), "T ^ F is true");
    assert!((f1 ^ f0).is_false(), "F ^ F is false");
    assert!((t1 ^ t0).is_false(), "T ^ T is false");

    assert_eq!(t0.then_some(0), Some(0));
    assert_eq!(t0.then(|| 1), Some(1));

    assert!((t0 & true).is_true(), "T and True is true");
    assert!((t0 & false).is_false(), "T and False is false");

    let mut c = t0;
    assert!(c.is_true());
    c &= f0;
    assert!(c.is_false());
    c |= f0;
    assert!(c.is_false());
    c &= f1;
    assert!(c.is_false());
    c |= t1;
    assert!(c.is_true());
    c |= t0;
    assert!(c.is_true());
    c |= f0;
    assert!(c.is_true());

    let mut c = t0;
    assert!(c.is_true());
    c &= &f0;
    assert!(c.is_false());
    c |= &f0;
    assert!(c.is_false());
    c &= &f1;
    assert!(c.is_false());
    c |= &t1;
    assert!(c.is_true());
    c |= &t0;
    assert!(c.is_true());
    c |= &f0;
    assert!(c.is_true());

    let mut c = t0;
    assert!(c.is_true());
    c &= false;
    assert!(c.is_false());
    c |= false;
    assert!(c.is_false());
    c &= false;
    assert!(c.is_false());
    c |= true;
    assert!(c.is_true());
    c |= false;
    assert!(c.is_true());

    Ok(())
}
