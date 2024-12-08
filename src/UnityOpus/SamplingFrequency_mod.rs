#[cfg(feature = "UnityOpus+SamplingFrequency")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingFrequency {
    Frequency_12000 = 12000i32,
    Frequency_16000 = 16000i32,
    Frequency_24000 = 24000i32,
    Frequency_48000 = 48000i32,
    Frequency_8000 = 8000i32,
}
#[cfg(feature = "UnityOpus+SamplingFrequency")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityOpus::SamplingFrequency => "UnityOpus"
    ."SamplingFrequency"
);
