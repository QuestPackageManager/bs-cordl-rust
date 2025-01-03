#[cfg(feature = "LIV+SDK+Unity+SDKTransform")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SDKTransform {
    pub localPosition: crate::LIV::SDK::Unity::SDKVector3,
    pub localRotation: crate::LIV::SDK::Unity::SDKQuaternion,
    pub localScale: crate::LIV::SDK::Unity::SDKVector3,
}
#[cfg(feature = "LIV+SDK+Unity+SDKTransform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKTransform => "LIV.SDK.Unity"
    ."SDKTransform"
);
#[cfg(feature = "LIV+SDK+Unity+SDKTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKTransform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKTransform")]
impl crate::LIV::SDK::Unity::SDKTransform {
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
        crate::LIV::SDK::Unity::SDKTransform,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_empty", ())?;
        Ok(__cordl_ret.into())
    }
}
