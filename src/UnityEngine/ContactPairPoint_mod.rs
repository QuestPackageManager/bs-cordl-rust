#[cfg(feature = "UnityEngine+ContactPairPoint")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ContactPairPoint {
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_Separation: f32,
    pub m_Normal: crate::UnityEngine::Vector3,
    pub m_InternalFaceIndex0: u32,
    pub m_Impulse: crate::UnityEngine::Vector3,
    pub m_InternalFaceIndex1: u32,
}
#[cfg(feature = "UnityEngine+ContactPairPoint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ContactPairPoint => "UnityEngine"
    ."ContactPairPoint"
);
#[cfg(feature = "UnityEngine+ContactPairPoint")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ContactPairPoint {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ContactPairPoint")]
impl crate::UnityEngine::ContactPairPoint {
    pub fn get_Separation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Separation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Impulse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Impulse",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Normal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Normal",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
