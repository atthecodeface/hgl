//a Imports
use std::pin::Pin;

use crate::index_vec::make_index;
use crate::index_vec::VecWithIndex;

//a Name
make_index!(StringIndex, usize);

//a StringsWithIndex
//tp StringsWithIndex
#[derive(Debug, Default)]
pub struct StringsWithIndex {
    // Note that VecWithIndex requires the key to be static (as
    // probably does a HashMap)
    names: VecWithIndex<&'static str, StringIndex, Pin<String>>,
}

//ip Index<Name> for StringsWithIndex
impl std::ops::Index<StringIndex> for StringsWithIndex {
    type Output = str;
    fn index(&self, p: StringIndex) -> &str {
        Pin::into_inner(self.names[p].as_ref())
    }
}

//ip StringsWithIndex
impl StringsWithIndex {
    //mi add
    pub fn add<S: Into<String>>(&mut self, s: S) -> StringIndex {
        let s = s.into();
        let pinned_string = Pin::new(s);
        let pinned_str: &str = unsafe { std::mem::transmute::<_, _>(pinned_string.as_ref()) };
        self.names.add(pinned_str, pinned_string)
    }

    //mi find_or_add
    pub fn find_or_add<S: Into<String> + AsRef<str>>(&mut self, s: S) -> StringIndex {
        if let Some(p) = self.find_name(s.as_ref()) {
            p
        } else {
            self.add(s)
        }
    }

    //mp find_name
    pub fn find_name(&self, s: &str) -> Option<StringIndex> {
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
        let temp_str: &str = unsafe { std::mem::transmute::<_, _>(s) };
        self.names.find_key(&temp_str)
    }

    //mp fmt_name
    pub fn fmt_name(
        &self,
        fmt: &mut std::fmt::Formatter,
        name: StringIndex,
    ) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self[name])
    }
}
