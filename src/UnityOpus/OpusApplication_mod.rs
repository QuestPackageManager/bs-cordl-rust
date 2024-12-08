#[cfg(feature = "UnityOpus+OpusApplication")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpusApplication {
    Audio = 2049i32,
    RestrictedLowDelay = 2051i32,
    VoIP = 2048i32,
}
#[cfg(feature = "UnityOpus+OpusApplication")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityOpus::OpusApplication => "UnityOpus"
    ."OpusApplication"
);
