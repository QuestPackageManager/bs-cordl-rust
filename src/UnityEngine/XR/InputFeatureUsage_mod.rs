#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputFeatureUsage {
    pub m_Name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_InternalType: crate::UnityEngine::XR::InputFeatureType,
}
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::InputFeatureUsage =>
    "UnityEngine.XR"."InputFeatureUsage"
);
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::InputFeatureUsage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
impl crate::UnityEngine::XR::InputFeatureUsage {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_InputFeatureUsage1(
        &mut self,
        other: crate::UnityEngine::XR::InputFeatureUsage,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn get_internalType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::InputFeatureType> {
        let __cordl_ret: crate::UnityEngine::XR::InputFeatureType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_internalType",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_name",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
