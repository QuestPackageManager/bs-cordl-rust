#[cfg(feature = "UnityEngine+Resolution")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    pub fn get_height(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_height",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_refreshRateRatio(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RefreshRate> {
        let __cordl_ret: crate::UnityEngine::RefreshRate = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_refreshRateRatio",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_width",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
