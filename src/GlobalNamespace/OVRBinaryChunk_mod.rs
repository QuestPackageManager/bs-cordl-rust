#[cfg(feature = "OVRBinaryChunk")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRBinaryChunk {
    pub chunkStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub chunkLength: u32,
    pub chunkStart: i64,
}
#[cfg(feature = "OVRBinaryChunk")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRBinaryChunk => ""
    ."OVRBinaryChunk"
);
#[cfg(feature = "OVRBinaryChunk")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRBinaryChunk {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRBinaryChunk")]
impl crate::GlobalNamespace::OVRBinaryChunk {}
