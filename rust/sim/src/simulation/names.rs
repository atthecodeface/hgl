//a Imports
use std::collections::HashMap;
use std::pin::Pin;

//a Name
//tp Name
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Name(usize);

//ip From <usize> for Name
impl From<usize> for Name {
    fn from(p: usize) -> Name {
        Name(p)
    }
}

//a SimNsName
//tp SimNsName
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct SimNsName(usize);

//ip From <usize> for SimNsName
impl From<usize> for SimNsName {
    fn from(f: usize) -> SimNsName {
        SimNsName(f)
    }
}

//ip  SimNsName
impl SimNsName {
    pub fn is_root(&self) -> bool {
        self.0 == 0
    }
}

//a NsName
//tp NsName
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NsName {
    namespace: SimNsName,
    name: Name,
}

//ip  NsName
impl NsName {
    pub fn is_root(&self) -> bool {
        self.namespace.0 == 0
    }
    pub fn name(&self) -> Name {
        self.name
    }
    pub fn namespace(&self) -> SimNsName {
        self.namespace
    }
}

//ip PartialOrd for NsName
impl std::cmp::PartialOrd for NsName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

//ip Ord for NsName
impl std::cmp::Ord for NsName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match self.namespace.cmp(&other.namespace) {
            Equal => self.name.cmp(&other.name),
            c => c,
        }
    }
}

//ip From (SimNsName, Name) for NsName
impl From<(SimNsName, Name)> for NsName {
    fn from((namespace, name): (SimNsName, Name)) -> NsName {
        NsName { namespace, name }
    }
}

//a Names
//tp Names
pub struct Names {
    pool: Vec<Pin<String>>,
    pool_index: HashMap<&'static str, Name>,
    namespace_names: Vec<NsName>,
    namespace_name_index: HashMap<NsName, SimNsName>,
}

//ip Default for Names
impl std::default::Default for Names {
    fn default() -> Self {
        let pool = vec![];
        let pool_index = HashMap::default();
        let namespace_names = vec![NsName::default()];
        let namespace_name_index = HashMap::default();
        let mut s = Self {
            pool,
            pool_index,
            namespace_names,
            namespace_name_index,
        };
        s.add_string("");
        s
    }
}

//ip Index<Name> for Names
impl std::ops::Index<Name> for Names {
    type Output = str;
    fn index(&self, p: Name) -> &str {
        Pin::into_inner(self.pool[p.0].as_ref())
    }
}

//ip Index<SimNsName> for Names
impl std::ops::Index<SimNsName> for Names {
    type Output = NsName;
    fn index(&self, n: SimNsName) -> &NsName {
        &self.namespace_names[n.0]
    }
}

//ip Names
impl Names {
    pub fn root_namespace(&self) -> NsName {
        self.namespace_names[0]
    }

    fn add_full_name(&mut self, f: NsName) -> SimNsName {
        let n = self.namespace_names.len().into();
        self.namespace_names.push(f);
        self.namespace_name_index.insert(f, n);
        n
    }

    fn get_full_name(&self, f: &NsName) -> Option<SimNsName> {
        self.namespace_name_index.get(f).copied()
    }

    pub fn insert_full_name(
        &mut self,
        namespace: SimNsName,
        name: &str,
    ) -> Result<SimNsName, SimNsName> {
        let name = self.insert_pool(name);
        let full_name = (namespace, name).into();
        if let Some(p) = self.get_full_name(&full_name) {
            Err(p)
        } else {
            Ok(self.add_full_name(full_name))
        }
    }

    fn add_string<S: Into<String>>(&mut self, s: S) -> Name {
        let s = s.into();
        let n = self.pool.len();
        self.pool.push(Pin::new(s));
        let pn = n.into();
        let s: &str = &self.pool[n];
        let s: &'static str = unsafe { std::mem::transmute::<_, _>(s) };
        self.pool_index.insert(s, pn);
        pn
    }

    fn get_pool(&self, s: &str) -> Option<Name> {
        self.pool_index.get(s).copied()
    }

    fn insert_pool<S: Into<String> + AsRef<str>>(&mut self, s: S) -> Name {
        if let Some(p) = self.get_pool(s.as_ref()) {
            p
        } else {
            self.add_string(s)
        }
    }

    pub fn add_name<S: Into<String> + AsRef<str>>(&mut self, s: S) -> Name {
        self.insert_pool(s)
    }

    pub fn fmt_name(
        &self,
        fmt: &mut std::fmt::Formatter,
        name: Name,
    ) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self[name])
    }
    pub fn fmt_ns_name(
        &self,
        fmt: &mut std::fmt::Formatter,
        name: SimNsName,
    ) -> Result<(), std::fmt::Error> {
        let ns_name = self[name];
        if !ns_name.namespace.is_root() {
            self.fmt_ns_name(fmt, ns_name.namespace)?;
            fmt.write_str(".")?;
        }
        self.fmt_name(fmt, ns_name.name())
    }
}

//a NamespaceStack
#[derive(Debug, Default, Clone)]
pub struct NamespaceStack {
    stack: Vec<SimNsName>,
}
impl NamespaceStack {
    pub fn top(&self) -> SimNsName {
        if let Some(s) = self.stack.last() {
            *s
        } else {
            SimNsName::default()
        }
    }
    #[track_caller]
    pub fn push(&mut self, f: SimNsName) {
        assert!(f.0 != 0, "Cannot push the root namespace");
        self.stack.push(f);
    }
    #[track_caller]
    pub fn pop(&mut self) -> SimNsName {
        assert!(
            !self.stack.is_empty(),
            "Empty namespace stack cannot be popped"
        );
        self.stack.pop().unwrap()
    }
}
