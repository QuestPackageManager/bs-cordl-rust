#[cfg(feature = "NoteSpawnData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NoteSpawnData {
    pub moveStartOffset: crate::UnityEngine::Vector3,
    pub moveEndOffset: crate::UnityEngine::Vector3,
    pub jumpEndOffset: crate::UnityEngine::Vector3,
    pub gravityBase: f32,
}
#[cfg(feature = "NoteSpawnData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteSpawnData => ""
    ."NoteSpawnData"
);
#[cfg(feature = "NoteSpawnData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::NoteSpawnData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "NoteSpawnData")]
impl crate::GlobalNamespace::NoteSpawnData {
    pub fn _ctor(
        &mut self,
        moveStartOffset: crate::UnityEngine::Vector3,
        moveEndOffset: crate::UnityEngine::Vector3,
        jumpEndOffset: crate::UnityEngine::Vector3,
        gravityBase: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (moveStartOffset, moveEndOffset, jumpEndOffset, gravityBase),
        )?;
        Ok(__cordl_ret.into())
    }
}
