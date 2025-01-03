#[cfg(feature = "MultiplayerAvatarData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MultiplayerAvatarData {
    pub avatarTypeIdentifierHash: u32,
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "MultiplayerAvatarData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerAvatarData => ""
    ."MultiplayerAvatarData"
);
#[cfg(feature = "MultiplayerAvatarData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::MultiplayerAvatarData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MultiplayerAvatarData")]
impl crate::GlobalNamespace::MultiplayerAvatarData {
    pub fn _ctor(
        &mut self,
        avatarTypeIdentifierHash: u32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (avatarTypeIdentifierHash, data),
        )?;
        Ok(__cordl_ret.into())
    }
}
