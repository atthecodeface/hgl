//a Imports
use std::pin::Pin;

use hgl_utils::index_vec::make_index;
use hgl_utils::index_vec::VecWithIndex;

//a Name
make_index!(Name, usize);

//a SimNsName
//tp SimNsName
make_index!(SimNsName, usize);

//ip SimNsName
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

//ip NsName
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
    names: VecWithIndex<&'static str, Name, Pin<String>>,
    namespace_names: VecWithIndex<NsName, SimNsName, NsName>,
}

//ip Default for Names
impl std::default::Default for Names {
    fn default() -> Self {
        let names = VecWithIndex::default();
        let namespace_names = VecWithIndex::default();
        let mut s = Self {
            namespace_names,
            names,
        };
        s.add_string("");
        s
    }
}

//ip Index<Name> for Names
impl std::ops::Index<Name> for Names {
    type Output = str;
    fn index(&self, p: Name) -> &str {
        Pin::into_inner(self.names[p].as_ref())
    }
}

//ip Index<SimNsName> for Names
impl std::ops::Index<SimNsName> for Names {
    type Output = NsName;
    fn index(&self, n: SimNsName) -> &NsName {
        &self.namespace_names[n]
    }
}

//ip Names
impl Names {
    pub fn root_namespace(&self) -> NsName {
        self.namespace_names.first().copied().unwrap()
    }

    fn add_full_name(&mut self, f: NsName) -> SimNsName {
        self.namespace_names.add(f, f)
    }

    fn get_full_name(&self, f: NsName) -> Option<SimNsName> {
        self.namespace_names.get(&f)
    }

    pub fn insert_full_name(
        &mut self,
        namespace: SimNsName,
        name: &str,
    ) -> Result<SimNsName, SimNsName> {
        let name = self.find_or_add_to_pool(name);
        let full_name = (namespace, name).into();
        if let Some(p) = self.get_full_name(full_name) {
            Err(p)
        } else {
            Ok(self.add_full_name(full_name))
        }
    }

    //mi add_string
    fn add_string<S: Into<String>>(&mut self, s: S) -> Name {
        let s = s.into();
        let pinned_string = Pin::new(s);
        let pinned_str: &'static str =
            unsafe { std::mem::transmute::<_, _>(pinned_string.as_ref()) };
        self.names.add(pinned_str, pinned_string)
    }

    //mi find_or_add_to_pool
    fn find_or_add_to_pool<S: Into<String> + AsRef<str>>(&mut self, s: S) -> Name {
        if let Some(p) = self.find_name(s.as_ref()) {
            p
        } else {
            self.add_string(s)
        }
    }

    //mp add_name
    pub fn add_name<S: Into<String> + AsRef<str>>(&mut self, s: S) -> Name {
        self.find_or_add_to_pool(s)
    }

    //mp find_name
    pub fn find_name(&self, s: &str) -> Option<Name> {
        // SAFETY:
        //
        // s is &'fn str - i.e. must be live for this function
        //
        // names.get() *borrows* &'static str, but it's use cannot
        // outlive this function
        //
        // So names.get() when names has an &'static str is its key needs to have its lifetime extended
        //
        // No
        let temp_str: &'static str = unsafe { std::mem::transmute::<_, _>(s) };
        self.names.get(&temp_str)
    }

    //mp fmt_name
    pub fn fmt_name(
        &self,
        fmt: &mut std::fmt::Formatter,
        name: Name,
    ) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self[name])
    }

    //mp fmt_ns_name
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
