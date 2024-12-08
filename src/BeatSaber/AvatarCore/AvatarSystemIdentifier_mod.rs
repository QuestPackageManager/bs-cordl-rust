#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AvatarSystemIdentifier {
    pub value: *mut crate::System::String,
    pub hash: u32,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::AvatarSystemIdentifier =>
    "BeatSaber.AvatarCore"."AvatarSystemIdentifier"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::AvatarCore::AvatarSystemIdentifier {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
impl crate::BeatSaber::AvatarCore::AvatarSystemIdentifier {
    pub fn Equals_AvatarSystemIdentifier0(
        &mut self,
        other: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        avatarSystemTypeIdentifier: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (avatarSystemTypeIdentifier),
        )?;
        Ok(__cordl_ret)
    }
}