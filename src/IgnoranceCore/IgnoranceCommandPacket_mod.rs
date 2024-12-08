#[cfg(feature = "IgnoranceCore+IgnoranceCommandPacket")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IgnoranceCommandPacket {
    pub Type: crate::IgnoranceCore::IgnoranceCommandType,
    pub PeerId: u32,
}
#[cfg(feature = "IgnoranceCore+IgnoranceCommandPacket")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceCommandPacket =>
    "IgnoranceCore"."IgnoranceCommandPacket"
);
#[cfg(feature = "IgnoranceCore+IgnoranceCommandPacket")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::IgnoranceCore::IgnoranceCommandPacket {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceCommandPacket")]
impl crate::IgnoranceCore::IgnoranceCommandPacket {}
