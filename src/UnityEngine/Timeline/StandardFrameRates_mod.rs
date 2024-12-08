#[cfg(feature = "UnityEngine+Timeline+StandardFrameRates")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardFrameRates {
    Fps23_97 = 1i32,
    Fps24 = 0i32,
    Fps25 = 2i32,
    Fps29_97 = 4i32,
    Fps30 = 3i32,
    Fps50 = 5i32,
    Fps59_94 = 7i32,
    Fps60 = 6i32,
}
#[cfg(feature = "UnityEngine+Timeline+StandardFrameRates")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::StandardFrameRates =>
    "UnityEngine.Timeline"."StandardFrameRates"
);
