#[cfg(feature = "UnityEngine+FFTWindow")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FFTWindow {
    #[default]
    Blackman = 4i32,
    BlackmanHarris = 5i32,
    Hamming = 2i32,
    Hanning = 3i32,
    Rectangular = 0i32,
    Triangle = 1i32,
}
#[cfg(feature = "UnityEngine+FFTWindow")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FFTWindow => "UnityEngine"
    ."FFTWindow"
);
