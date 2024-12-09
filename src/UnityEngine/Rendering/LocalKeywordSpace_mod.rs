#[cfg(feature = "UnityEngine+Rendering+LocalKeywordSpace")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LocalKeywordSpace {
    pub m_KeywordSpace: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Rendering+LocalKeywordSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::LocalKeywordSpace =>
    "UnityEngine.Rendering"."LocalKeywordSpace"
);
#[cfg(feature = "UnityEngine+Rendering+LocalKeywordSpace")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::LocalKeywordSpace {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LocalKeywordSpace")]
impl crate::UnityEngine::Rendering::LocalKeywordSpace {
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
    pub fn Equals_LocalKeywordSpace1(
        &mut self,
        rhs: crate::UnityEngine::Rendering::LocalKeywordSpace,
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
}
