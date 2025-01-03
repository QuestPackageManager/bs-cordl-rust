#[cfg(feature = "SliderSpawnData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SliderSpawnData {
    pub headNoteOffset: crate::UnityEngine::Vector3,
    pub headGravityBase: f32,
    pub tailNoteOffset: crate::UnityEngine::Vector3,
    pub tailGravityBase: f32,
}
#[cfg(feature = "SliderSpawnData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderSpawnData => ""
    ."SliderSpawnData"
);
#[cfg(feature = "SliderSpawnData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SliderSpawnData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SliderSpawnData")]
impl crate::GlobalNamespace::SliderSpawnData {
    pub fn _ctor(
        &mut self,
        headNoteOffset: crate::UnityEngine::Vector3,
        headGravityBase: f32,
        tailNoteOffset: crate::UnityEngine::Vector3,
        tailGravityBase: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (headNoteOffset, headGravityBase, tailNoteOffset, tailGravityBase),
        )?;
        Ok(__cordl_ret.into())
    }
}
