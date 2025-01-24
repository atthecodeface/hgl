use std::collections::HashMap;
use std::pin::Pin;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct PoolIndex(usize);

/// For now derive Debug and Default, but not in the future
///
/// Quite probably the simulatino needs to be accessible from more
/// than one thread
pub struct FullName {
    namespace: usize,
    name: usize,
}

//ip FullName
impl FullName {
    //cp new
    pub fn new(_namespace: (), _name: &str) -> Result<Self, String> {
        Ok(Self {
            namespace: 0,
            name: 0,
        })
    }
}

pub struct Names {
    pool: Vec<Pin<String>>,
    index: HashMap<&'static str, PoolIndex>,
}

impl Names {
    pub fn new() -> Self {
        let pool = vec![];
        let index = HashMap::default();
        let mut s = Self { pool, index };
        s.add_string("");
        s
    }

    fn add_string<S: Into<String>>(&mut self, s: S) -> PoolIndex {
        let s = s.into();
        let n = self.pool.len();
        self.pool.push(Pin::new(s));
        let pn = PoolIndex(n);
        let s: &str = &self.pool[n];
        let s: &'static str = unsafe { std::mem::transmute::<_, _>(s) };
        self.index.insert(s, pn);
        pn
    }

    pub fn get(&self, s: &str) -> Option<PoolIndex> {
        self.index.get(s).copied()
    }

    pub fn insert<S: Into<String> + AsRef<str>>(&mut self, s: S) -> PoolIndex {
        if let Some(p) = self.get(s.as_ref()) {
            p
        } else {
            self.add_string(s)
        }
    }
}
