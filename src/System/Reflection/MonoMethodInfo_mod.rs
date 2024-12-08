#[cfg(feature = "System+Reflection+MonoMethodInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MonoMethodInfo {
    pub parent: *mut crate::System::Type,
    pub ret: *mut crate::System::Type,
    pub attrs: crate::System::Reflection::MethodAttributes,
    pub iattrs: crate::System::Reflection::MethodImplAttributes,
    pub callconv: crate::System::Reflection::CallingConventions,
}
#[cfg(feature = "System+Reflection+MonoMethodInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::MonoMethodInfo =>
    "System.Reflection"."MonoMethodInfo"
);
#[cfg(feature = "System+Reflection+MonoMethodInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Reflection::MonoMethodInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Reflection+MonoMethodInfo")]
impl crate::System::Reflection::MonoMethodInfo {}
