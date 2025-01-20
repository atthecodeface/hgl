pub(crate) trait U8Ops {
    fn bit<const NB: usize>(&self, n:usize) -> bool;
    fn bit_set<const NB: usize>(&mut self, n:usize, v:bool);
}

impl U8Ops for [u8] {
    #[track_caller]
    fn bit<const NB: usize>(&self, n:usize) -> bool {
        assert!(n < NB, "bit index out of range for vector");
        (self[n/8]>>(n&7))&1 != 0
    }

    #[track_caller]
    fn bit_set<const NB: usize>(&mut self, n:usize, v:bool) {
        assert!(n < NB, "bit index out of range for vector");
        let nb = n/8;
        let m = 1<<(n&7);
        if v {
            self[nb] = self[nb] | m;
        } else {
            self[nb] = self[nb] & !m;
        }
    }
}
