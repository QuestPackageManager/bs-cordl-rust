#[cfg(feature = "System+Runtime+CompilerServices+Ephemeron")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Ephemeron {
    pub key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+CompilerServices+Ephemeron")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::Ephemeron =>
    "System.Runtime.CompilerServices"."Ephemeron"
);
#[cfg(feature = "System+Runtime+CompilerServices+Ephemeron")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::Ephemeron {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+Ephemeron")]
impl crate::System::Runtime::CompilerServices::Ephemeron {}
