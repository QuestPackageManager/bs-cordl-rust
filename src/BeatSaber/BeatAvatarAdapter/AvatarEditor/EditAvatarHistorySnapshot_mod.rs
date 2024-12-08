#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarHistorySnapshot")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct EditAvatarHistorySnapshot {
    pub avatarEditPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    pub avatarData: *mut crate::BeatSaber::BeatAvatarSDK::AvatarData,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarHistorySnapshot")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot =>
    "BeatSaber.BeatAvatarAdapter.AvatarEditor"."EditAvatarHistorySnapshot"
);
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarHistorySnapshot")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarHistorySnapshot")]
impl crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot {
    pub fn _ctor(
        &mut self,
        avatarData: *mut crate::BeatSaber::BeatAvatarSDK::AvatarData,
        avatarEditPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (avatarData, avatarEditPart),
        )?;
        Ok(__cordl_ret)
    }
}
