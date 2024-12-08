#[cfg(feature = "UnityEngine+Resolution")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Resolution {
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_RefreshRate: crate::UnityEngine::RefreshRate,
}
#[cfg(feature = "UnityEngine+Resolution")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Resolution => "UnityEngine"
    ."Resolution"
);
#[cfg(feature = "UnityEngine+Resolution")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Resolution {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Resolution")]
impl crate::UnityEngine::Resolution {
    pub fn get_refreshRateRatio(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RefreshRate> {
        let __cordl_ret: crate::UnityEngine::RefreshRate = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_refreshRateRatio",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_height(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_height",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_width",
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
