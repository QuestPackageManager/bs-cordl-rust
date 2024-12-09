#[cfg(feature = "UnityEngine+Rendering+LocalKeyword")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LocalKeyword {
    pub m_SpaceInfo: crate::UnityEngine::Rendering::LocalKeywordSpace,
    pub m_Name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Index: u32,
}
#[cfg(feature = "UnityEngine+Rendering+LocalKeyword")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::LocalKeyword =>
    "UnityEngine.Rendering"."LocalKeyword"
);
#[cfg(feature = "UnityEngine+Rendering+LocalKeyword")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::LocalKeyword {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LocalKeyword")]
impl crate::UnityEngine::Rendering::LocalKeyword {
    pub fn Equals_Il2CppObject0(
        &mut self,
        o: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_LocalKeyword1(
        &mut self,
        rhs: crate::UnityEngine::Rendering::LocalKeyword,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (rhs),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
