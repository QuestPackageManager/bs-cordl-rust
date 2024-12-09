#[cfg(feature = "System+Reflection+MonoEventInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MonoEventInfo {
    pub declaring_type: *mut crate::System::Type,
    pub reflected_type: *mut crate::System::Type,
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub add_method: *mut crate::System::Reflection::MethodInfo,
    pub remove_method: *mut crate::System::Reflection::MethodInfo,
    pub raise_method: *mut crate::System::Reflection::MethodInfo,
    pub attrs: crate::System::Reflection::EventAttributes,
    pub other_methods: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Reflection::MethodInfo,
    >,
}
#[cfg(feature = "System+Reflection+MonoEventInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::MonoEventInfo =>
    "System.Reflection"."MonoEventInfo"
);
#[cfg(feature = "System+Reflection+MonoEventInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Reflection::MonoEventInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Reflection+MonoEventInfo")]
impl crate::System::Reflection::MonoEventInfo {}
