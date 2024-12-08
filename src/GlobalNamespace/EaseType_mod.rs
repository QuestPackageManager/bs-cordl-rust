#[cfg(feature = "EaseType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EaseType {
    BeatSaberInOutBack = 32i32,
    BeatSaberInOutBounce = 34i32,
    BeatSaberInOutElastic = 33i32,
    InBack = 23i32,
    InBounce = 29i32,
    InCirc = 20i32,
    InCubic = 8i32,
    InElastic = 26i32,
    InExpo = 17i32,
    InOutBack = 25i32,
    InOutBounce = 31i32,
    InOutCirc = 22i32,
    InOutCubic = 10i32,
    InOutElastic = 28i32,
    InOutExpo = 19i32,
    InOutQuad = 7i32,
    InOutQuart = 13i32,
    InOutQuint = 16i32,
    InOutSine = 4i32,
    InQuad = 5i32,
    InQuart = 11i32,
    InQuint = 14i32,
    InSine = 2i32,
    Linear = 1i32,
    None = 0i32,
    OutBack = 24i32,
    OutBounce = 30i32,
    OutCirc = 21i32,
    OutCubic = 9i32,
    OutElastic = 27i32,
    OutExpo = 18i32,
    OutQuad = 6i32,
    OutQuart = 12i32,
    OutQuint = 15i32,
    OutSine = 3i32,
}
#[cfg(feature = "EaseType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EaseType => ""."EaseType"
);
