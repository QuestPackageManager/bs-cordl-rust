#[cfg(feature = "System+Reflection+MonoPropertyInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MonoPropertyInfo {
    pub parent: *mut crate::System::Type,
    pub declaring_type: *mut crate::System::Type,
    pub name: *mut crate::System::String,
    pub get_method: *mut crate::System::Reflection::MethodInfo,
    pub set_method: *mut crate::System::Reflection::MethodInfo,
    pub attrs: crate::System::Reflection::PropertyAttributes,
}
#[cfg(feature = "System+Reflection+MonoPropertyInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::MonoPropertyInfo =>
    "System.Reflection"."MonoPropertyInfo"
);
#[cfg(feature = "System+Reflection+MonoPropertyInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Reflection::MonoPropertyInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Reflection+MonoPropertyInfo")]
impl crate::System::Reflection::MonoPropertyInfo {}
