#[cfg(feature = "BeatmapSaveDataCommon+EaseType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EaseType {
    #[default]
    BeatSaberInOutBack = 100i32,
    BeatSaberInOutBounce = 102i32,
    BeatSaberInOutElastic = 101i32,
    InBack = 22i32,
    InBounce = 28i32,
    InCirc = 19i32,
    InCubic = 7i32,
    InElastic = 25i32,
    InExpo = 16i32,
    InOutBack = 24i32,
    InOutBounce = 30i32,
    InOutCirc = 21i32,
    InOutCubic = 9i32,
    InOutElastic = 27i32,
    InOutExpo = 18i32,
    InOutQuad = 3i32,
    InOutQuart = 12i32,
    InOutQuint = 15i32,
    InOutSine = 6i32,
    InQuad = 1i32,
    InQuart = 10i32,
    InQuint = 13i32,
    InSine = 4i32,
    Linear = 0i32,
    None = -1i32,
    OutBack = 23i32,
    OutBounce = 29i32,
    OutCirc = 20i32,
    OutCubic = 8i32,
    OutElastic = 26i32,
    OutExpo = 17i32,
    OutQuad = 2i32,
    OutQuart = 11i32,
    OutQuint = 14i32,
    OutSine = 5i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+EaseType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::EaseType =>
    "BeatmapSaveDataCommon"."EaseType"
);
