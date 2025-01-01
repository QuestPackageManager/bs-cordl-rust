#[cfg(feature = "BeatSaber+GameSettings+Controller")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Controller {
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BeatSaber+GameSettings+Controller")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::Controller =>
    "BeatSaber.GameSettings"."Controller"
);
#[cfg(feature = "BeatSaber+GameSettings+Controller")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::GameSettings::Controller {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+Controller")]
impl crate::BeatSaber::GameSettings::Controller {
    pub fn HasDefaultValues(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasDefaultValues",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
