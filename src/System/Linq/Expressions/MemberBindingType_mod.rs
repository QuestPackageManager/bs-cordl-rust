#[cfg(feature = "System+Linq+Expressions+MemberBindingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberBindingType {
    Assignment = 0i32,
    ListBinding = 2i32,
    MemberBinding = 1i32,
}
#[cfg(feature = "System+Linq+Expressions+MemberBindingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::MemberBindingType =>
    "System.Linq.Expressions"."MemberBindingType"
);
