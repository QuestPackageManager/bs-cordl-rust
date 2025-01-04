#[cfg(feature = "UnityOpus+NumChannels")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NumChannels {
    #[default]
    Mono = 1i32,
    Stereo = 2i32,
}
#[cfg(feature = "UnityOpus+NumChannels")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityOpus::NumChannels => "UnityOpus"
    ."NumChannels"
);
