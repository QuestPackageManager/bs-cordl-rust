#[cfg(feature = "MultiplayerAvatarData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MultiplayerAvatarData {
    pub avatarTypeIdentifierHash: u32,
    pub data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "MultiplayerAvatarData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for MultiplayerAvatarData => ""."MultiplayerAvatarData"
);
#[cfg(feature = "MultiplayerAvatarData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for MultiplayerAvatarData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MultiplayerAvatarData")]
impl MultiplayerAvatarData {
    pub fn _ctor(
        &mut self,
        avatarTypeIdentifierHash: u32,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (avatarTypeIdentifierHash, data),
        )?;
        Ok(__cordl_ret)
    }
}
