#[cfg(feature = "BeatmapSaveDataVersion4+NoteJumpMovementSpeedEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NoteJumpMovementSpeedEvent {
    pub d: f32,
    pub p: i32,
    pub e: crate::BeatmapSaveDataCommon::EaseType,
}
#[cfg(feature = "BeatmapSaveDataVersion4+NoteJumpMovementSpeedEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion4::NoteJumpMovementSpeedEvent => "BeatmapSaveDataVersion4"
    ."NoteJumpMovementSpeedEvent"
);
#[cfg(feature = "BeatmapSaveDataVersion4+NoteJumpMovementSpeedEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::NoteJumpMovementSpeedEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+NoteJumpMovementSpeedEvent")]
impl crate::BeatmapSaveDataVersion4::NoteJumpMovementSpeedEvent {}
