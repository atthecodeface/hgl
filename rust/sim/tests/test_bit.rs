use hgl_sim::prelude::*;

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


    
