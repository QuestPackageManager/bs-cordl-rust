#[cfg(feature = "UnityEngine+UIElements+UIR+BMPAlloc")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BMPAlloc {
    pub page: i32,
    pub pageLine: u16,
    pub bitIndex: u8,
    pub ownedState: crate::UnityEngine::UIElements::UIR::OwnedState,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BMPAlloc")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::BMPAlloc =>
    "UnityEngine.UIElements.UIR"."BMPAlloc"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+BMPAlloc")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::BMPAlloc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BMPAlloc")]
impl crate::UnityEngine::UIElements::UIR::BMPAlloc {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
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
