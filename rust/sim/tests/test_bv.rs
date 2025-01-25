use hgl_sim::prelude::sim::*;

#[test]
fn format() -> Result<(), String> {
    for i in 0..128 {
        assert_eq!(
            &SimFormatValue::value_string(&Bv::<7>::of_u64(i), SIM_FMT_AS_BIN),
            &format!("{:07b}", i)
        );
        assert_eq!(
            &SimFormatValue::value_string(&Bv::<7>::of_u64(i), SIM_FMT_AS_HEX | SIM_FMT_AS_BIN),
            &format!("{:02x}", i)
        );
        assert_eq!(
            &SimFormatValue::value_string(&Bv::<7>::of_u64(i), SIM_FMT_AS_BIN | SIM_FMT_HDR),
            &format!("7b{:07b}", i)
        );
        assert_eq!(
            &SimFormatValue::value_string(
                &Bv::<7>::of_u64(i),
                SIM_FMT_AS_HEX | SIM_FMT_AS_BIN | SIM_FMT_HDR
            ),
            &format!("7h{:02x}", i)
        );
    }
    Ok(())
}

fn test_bvn<const N: usize>() -> Result<(), String>
where
    BvN<N>: IsBv,
{
    let bv = Bv::<N>::default();
    assert!(bv.is_zero(), "Bit vector default value is zero");
    let mut bv2 = bv;
    bv2.bit_set(0, true);
    assert!(bv.is_zero(), "Bit vector default value is zero");
    assert!(!bv2.is_zero(), "Bit vector with bottom bit set is not zero");

    bv2 += bv2;
    let mut v = 2;
    if N == 1 {
        v = 0;
        assert!(
            bv2.is_zero(),
            "Bit vector of should-be-2 must be zero {bv2:?}"
        );
    } else {
        assert!(!bv2.is_zero(), "Bit vector of 1+1 must be not zero {bv2:?}");
    }

    if N < 8 {
        assert_eq!(
            &format!("{}b{:0width$b}", N, v, width = N),
            &format!("{:?}", bv2),
            "Mismatch of debug"
        );
    } else {
        assert_eq!(
            &format!("{}h{:0width$x}", N, v, width = (N + 3) / 4),
            &format!("{:?}", bv2),
            "Mismatch of debug"
        );
    }
    Ok(())
}

#[test]
fn construct_1() -> Result<(), String> {
    test_bvn::<1>()?;
    Ok(())
}

#[test]
fn construct_2() -> Result<(), String> {
    test_bvn::<2>()?;
    Ok(())
}

#[test]
fn construct_3() -> Result<(), String> {
    test_bvn::<3>()?;
    Ok(())
}

#[test]
fn construct_32() -> Result<(), String> {
    test_bvn::<32>()?;
    Ok(())
}

#[test]
fn construct_64() -> Result<(), String> {
    test_bvn::<64>()?;
    Ok(())
}

#[test]
fn construct_128() -> Result<(), String> {
    test_bvn::<128>()?;
    Ok(())
}
