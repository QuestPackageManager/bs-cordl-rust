#[cfg(feature = "LIV+SDK+Unity+SDKApplicationOutput")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SDKApplicationOutput {
    pub supportedFeatures: crate::LIV::SDK::Unity::FEATURES,
    pub engineName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub engineVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub applicationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub applicationVersion: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub xrDeviceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub graphicsAPI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub sdkID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub sdkVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "LIV+SDK+Unity+SDKApplicationOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKApplicationOutput =>
    "LIV.SDK.Unity"."SDKApplicationOutput"
);
#[cfg(feature = "LIV+SDK+Unity+SDKApplicationOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKApplicationOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKApplicationOutput")]
impl crate::LIV::SDK::Unity::SDKApplicationOutput {
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
        crate::LIV::SDK::Unity::SDKApplicationOutput,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKApplicationOutput = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_empty", ())?;
        Ok(__cordl_ret.into())
    }
}
