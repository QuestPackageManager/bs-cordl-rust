#[cfg(feature = "LIV+SDK+Unity+SDKPriority")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKPriority {
    pub pose: i8,
    pub clipPlane: i8,
    pub stage: i8,
    pub resolution: i8,
    pub feature: i8,
    pub nearFarAdjustment: i8,
    pub groundPlane: i8,
    pub reserved2: i8,
}
#[cfg(feature = "LIV+SDK+Unity+SDKPriority")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKPriority => "LIV.SDK.Unity"
    ."SDKPriority"
);
#[cfg(feature = "LIV+SDK+Unity+SDKPriority")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LIV::SDK::Unity::SDKPriority {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKPriority")]
impl crate::LIV::SDK::Unity::SDKPriority {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
