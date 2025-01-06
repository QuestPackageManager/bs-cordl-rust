#[cfg(feature = "LIV+SDK+Unity+SDKResolution")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SDKResolution {
    pub width: i32,
    pub height: i32,
}
#[cfg(feature = "LIV+SDK+Unity+SDKResolution")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKResolution =>
    "LIV.SDK.Unity"."SDKResolution"
);
#[cfg(feature = "LIV+SDK+Unity+SDKResolution")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKResolution {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKResolution")]
impl crate::LIV::SDK::Unity::SDKResolution {
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
    pub fn get_zero() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKResolution,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKResolution = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_zero", ())?;
        Ok(__cordl_ret.into())
    }
}
