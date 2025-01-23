pub struct SubInstance {}

/// For now derive Debug and Default, but not in the future
///
/// Quite probably the simulatino needs to be accessible from more
/// than one thread
pub struct FullName {
    namespace: String,
    name: String,
}

//ip FullName
impl FullName {
    //cp new
    pub fn new(_namespace: (), name: &str) -> Result<Self, String> {
        Ok(Self {
            namespace: "".into(),
            name: name.into(),
        })
    }
}
