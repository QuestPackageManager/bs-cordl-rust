#[cfg(feature = "OVRChunkType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRChunkType {
    BIN = 5130562i32,
    JSON = 1313821514i32,
}
#[cfg(feature = "OVRChunkType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for OVRChunkType => ""."OVRChunkType"
);
