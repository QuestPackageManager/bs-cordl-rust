#[cfg(feature = "BeatmapSaveDataVersion4+LightRotationEventBox")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LightRotationEventBox {
    pub w: f32,
    pub d: crate::BeatmapSaveDataCommon::DistributionParamType,
    pub s: f32,
    pub t: crate::BeatmapSaveDataCommon::DistributionParamType,
    pub b: i32,
    pub e: crate::BeatmapSaveDataCommon::EaseType,
    pub a: crate::BeatmapSaveDataCommon::Axis,
    pub f: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightRotationEventBox")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::LightRotationEventBox
    => "BeatmapSaveDataVersion4"."LightRotationEventBox"
);
#[cfg(feature = "BeatmapSaveDataVersion4+LightRotationEventBox")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::LightRotationEventBox {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightRotationEventBox")]
impl crate::BeatmapSaveDataVersion4::LightRotationEventBox {}
