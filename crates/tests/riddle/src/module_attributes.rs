#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
pub struct Type {
    pub field: i32,
}
impl Copy for Type {}
impl Clone for Type {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Type").field("field", &self.field).finish()
    }
}
impl windows_core::TypeKind for Type {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Type {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Type;i4)");
}
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}
impl Eq for Type {}
impl Default for Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
