#[cfg(feature = "IgnoranceCore+IgnoranceConnectionEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IgnoranceConnectionEvent {
    pub EventType: u8,
    pub Port: u16,
    pub NativePeerId: u32,
    pub IP: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "IgnoranceCore+IgnoranceConnectionEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceConnectionEvent =>
    "IgnoranceCore"."IgnoranceConnectionEvent"
);
#[cfg(feature = "IgnoranceCore+IgnoranceConnectionEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::IgnoranceCore::IgnoranceConnectionEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceConnectionEvent")]
impl crate::IgnoranceCore::IgnoranceConnectionEvent {}
