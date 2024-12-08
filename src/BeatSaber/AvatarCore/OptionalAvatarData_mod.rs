#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OptionalAvatarData {
    pub dataType: u32,
    pub length: i32,
    pub data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::OptionalAvatarData =>
    "BeatSaber.AvatarCore"."OptionalAvatarData"
);
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
impl crate::BeatSaber::AvatarCore::OptionalAvatarData {
    pub const kMaxDataSize: i32 = 2048i32;
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::AvatarCore::OptionalAvatarData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
}
