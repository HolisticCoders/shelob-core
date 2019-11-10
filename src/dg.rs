use std::fmt;

pub trait DGItem {
    fn path(&self) -> DGPath;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DGITemType {
    Node,
    Plug,
    Socket,
    Attribute,
}

#[derive(Debug, Eq, Hash)]
pub struct DGPath {
    pub path: String,
    pub item_type: DGITemType,
}

impl fmt::Display for DGPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

// Implement <DGPath> == <DGPath> comparisons
impl PartialEq for DGPath {
    fn eq(&self, other: &Self) -> bool {
        self.path == *other.path
    }
}

// Implement <DGPath> == <String> comparisons
impl PartialEq<String> for DGPath {
    fn eq(&self, other: &String) -> bool {
        self.path == *other
    }
}

// Implement <DGPath> == <&str> comparisons
impl PartialEq<&str> for DGPath {
    fn eq(&self, other: &&str) -> bool {
        self.path == *other
    }
}
