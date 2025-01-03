#[cfg(feature = "LIV+SDK+Unity+SDKTexture")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SDKTexture {
    pub id: crate::LIV::SDK::Unity::TEXTURE_ID,
    pub texturePtr: crate::System::IntPtr,
    pub SharedHandle: crate::System::IntPtr,
    pub device: crate::LIV::SDK::Unity::TEXTURE_DEVICE,
    pub dummy: i32,
    pub _cordl_type: crate::LIV::SDK::Unity::TEXTURE_TYPE,
    pub format: crate::LIV::SDK::Unity::TEXTURE_FORMAT,
    pub colorSpace: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
    pub width: i32,
    pub height: i32,
}
#[cfg(feature = "LIV+SDK+Unity+SDKTexture")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKTexture => "LIV.SDK.Unity"
    ."SDKTexture"
);
#[cfg(feature = "LIV+SDK+Unity+SDKTexture")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LIV::SDK::Unity::SDKTexture {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKTexture")]
impl crate::LIV::SDK::Unity::SDKTexture {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_empty() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKTexture,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKTexture = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_empty", ())?;
        Ok(__cordl_ret.into())
    }
}
