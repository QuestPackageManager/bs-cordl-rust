#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarHistorySnapshot")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EditAvatarHistorySnapshot {
    pub avatarEditPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    pub avatarData: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarData,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarHistorySnapshot")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarAdapter.AvatarEditor";
    const CLASS_NAME: &'static str = "EditAvatarHistorySnapshot";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
        avatarEditPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (avatarData, avatarEditPart),
        )?;
        Ok(__cordl_ret.into())
    }
}
