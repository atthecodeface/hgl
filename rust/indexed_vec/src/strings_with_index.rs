//a Imports
use std::pin::Pin;

use crate::make_index;
use crate::VecWithIndex;

//a Name
make_index!(StringIndex, usize);

//a StringsWithIndex
//tp StringsWithIndex
#[derive(Debug, Default)]
pub struct StringsWithIndex {
    // Note that VecWithIndex requires the key to be static (as
    // probably does a HashMap)
    strings: VecWithIndex<&'static str, StringIndex, Pin<String>>,
}

//ip Index<Name> for StringsWithIndex
impl std::ops::Index<StringIndex> for StringsWithIndex {
    type Output = str;
    fn index(&self, p: StringIndex) -> &str {
        Pin::into_inner(self.strings[p].as_ref())
    }
}

//ip StringsWithIndex
impl StringsWithIndex {
    //mi add
    /// Add string to the indexed array; must only be issued if find returns false
    fn add<S: Into<String>>(&mut self, s: S) -> StringIndex {
        let s = s.into();
        let pinned_string = Pin::new(s);
        let pinned_str: &str = unsafe { std::mem::transmute::<_, _>(pinned_string.as_ref()) };
        self.strings.find_or_add(pinned_str, |_| pinned_string).1
    }

    //mp find_or_add
    #[must_use]
    pub fn find_or_add<S: Into<String> + AsRef<str>>(&mut self, s: S) -> (bool, StringIndex) {
        if let Some(p) = self.find_string(s.as_ref()) {
            (true, p)
        } else {
            (false, self.add(s))
        }
    }

    //mp insert
    /// Add data to the array and index, but only if it is not present
    ///
    /// If it is already present, return an Err
    pub fn insert<S: Into<String> + AsRef<str>>(
        &mut self,
        s: S,
    ) -> Result<StringIndex, StringIndex> {
        let (found, index) = self.find_or_add(s);
        if found {
            Err(index)
        } else {
            Ok(index)
        }
    }

    //mp find_string
    pub fn find_string(&self, s: &str) -> Option<StringIndex> {
        // SAFETY:
        //
        // s is &'fn str - i.e. must be live for this function
        //
        // strings.get() *borrows* &'static str, but it's use cannot
        // outlive this function
        //
        // So strings.get() when strings has an &'static str is its key needs to have its lifetime extended
        let temp_str: &str = unsafe { std::mem::transmute::<_, _>(s) };
        self.strings.find_key(&temp_str)
    }

    //mp fmt_string
    pub fn fmt_string(
        &self,
        fmt: &mut std::fmt::Formatter,
        name: StringIndex,
    ) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self[name])
    }
}
