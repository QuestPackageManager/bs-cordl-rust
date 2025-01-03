#[cfg(feature = "System+Reflection+ParameterModifier")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParameterModifier {
    pub _byRef: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
}
#[cfg(feature = "System+Reflection+ParameterModifier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::ParameterModifier =>
    "System.Reflection"."ParameterModifier"
);
#[cfg(feature = "System+Reflection+ParameterModifier")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Reflection::ParameterModifier {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Reflection+ParameterModifier")]
impl crate::System::Reflection::ParameterModifier {}
