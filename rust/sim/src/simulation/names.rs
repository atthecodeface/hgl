//a Imports
use hgl_indexed_vec::make_index;
use hgl_indexed_vec::{StringIndex, StringsWithIndex, VecWithIndex};

//a Name
pub type Name = StringIndex;

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

//a Name formatters
//tp NameFmt
/// A formatter for full namespace names
///
///
pub struct NameFmt<'f, 'n>(&'f Names<'n>, Name);

//ip Display for NameFmt
impl std::fmt::Display for NameFmt<'_, '_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.0.fmt_name(fmt, self.1)
    }
}

//tp NsNameFmt
/// A formatter for full namespace names
///
///
pub struct NsNameFmt<'f, 'n>(&'f Names<'n>, SimNsName);

//ip Display for NsNameFmt
impl std::fmt::Display for NsNameFmt<'_, '_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.0.fmt_ns_name(fmt, self.1)
    }
}

//a Names
//tp Names
pub struct Names<'a> {
    names: StringsWithIndex<'a>,
    namespace_names: VecWithIndex<'a, NsName, SimNsName, NsName>,
}

//ip Default for Names
impl std::default::Default for Names<'_> {
    fn default() -> Self {
        let mut names = StringsWithIndex::default();
        let _ = names.find_or_add("");
        let namespace_names = VecWithIndex::default();
        Self {
            names,
            namespace_names,
        }
    }
}

//ip Index<Name> for Names
impl std::ops::Index<Name> for Names<'_> {
    type Output = str;
    fn index(&self, p: Name) -> &str {
        &self.names[p]
    }
}

//ip Index<SimNsName> for Names
impl std::ops::Index<SimNsName> for Names<'_> {
    type Output = NsName;
    fn index(&self, n: SimNsName) -> &NsName {
        &self.namespace_names[n]
    }
}

//ip Names
impl Names<'_> {
    pub fn root_namespace(&self) -> NsName {
        self.namespace_names.first().copied().unwrap()
    }

    fn add_full_name(&mut self, f: NsName) -> Result<SimNsName, String> {
        self.namespace_names
            .insert(f, |_| f)
            .map_err(|_| format!("Duplicate name in namespace"))
    }

    fn get_full_name(&self, f: NsName) -> Option<SimNsName> {
        self.namespace_names.find_key(&f)
    }

    pub fn insert_full_name(
        &mut self,
        namespace: SimNsName,
        name: &str,
    ) -> Result<SimNsName, SimNsName> {
        let name = self.add_name(name);
        let full_name = (namespace, name).into();
        if let Some(p) = self.get_full_name(full_name) {
            Err(p)
        } else {
            Ok(self.add_full_name(full_name).unwrap())
        }
    }

    //mp add_name
    pub fn add_name<S: Into<String> + AsRef<str>>(&mut self, s: S) -> Name {
        self.names.find_or_add(s).1
    }

    //mp find_name
    pub fn find_name(&self, s: &str) -> Option<Name> {
        self.names.find_string(s)
    }

    //mp fmt_name
    pub fn fmt_name(
        &self,
        fmt: &mut std::fmt::Formatter,
        name: Name,
    ) -> Result<(), std::fmt::Error> {
        self.names.fmt_string(fmt, name)
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

    //mp ns_name_fmt
    pub fn ns_name_fmt(&self, name: SimNsName) -> NsNameFmt {
        NsNameFmt(self, name)
    }

    //mp name_fmt
    pub fn name_fmt(&self, name: Name) -> NameFmt {
        NameFmt(self, name)
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
