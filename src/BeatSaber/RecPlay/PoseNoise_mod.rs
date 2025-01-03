#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PoseNoise {
    pub frequency: f32,
    pub _cordl_move: f32,
    pub rotate: f32,
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PoseNoise =>
    "BeatSaber.RecPlay"."PoseNoise"
);
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::RecPlay::PoseNoise {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
impl crate::BeatSaber::RecPlay::PoseNoise {
    pub fn Sample(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Sample",
            (_cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SampleLemniscateOfBernoulli(
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SampleLemniscateOfBernoulli", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
}
