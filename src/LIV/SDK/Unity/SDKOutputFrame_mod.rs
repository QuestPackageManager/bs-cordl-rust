#[cfg(feature = "LIV+SDK+Unity+SDKOutputFrame")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKOutputFrame {
    pub renderingPipeline: crate::LIV::SDK::Unity::RENDERING_PIPELINE,
    pub trackedSpace: crate::LIV::SDK::Unity::SDKTrackedSpace,
}
#[cfg(feature = "LIV+SDK+Unity+SDKOutputFrame")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKOutputFrame =>
    "LIV.SDK.Unity"."SDKOutputFrame"
);
#[cfg(feature = "LIV+SDK+Unity+SDKOutputFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKOutputFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKOutputFrame")]
impl crate::LIV::SDK::Unity::SDKOutputFrame {
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
}
