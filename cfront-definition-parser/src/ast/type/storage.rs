#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StorageClassSpec {
    Auto, 
    Register, 
    Static, 
    Extern, 
    Typedef, 
}
