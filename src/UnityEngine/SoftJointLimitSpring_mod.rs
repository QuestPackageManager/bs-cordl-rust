#[cfg(feature = "UnityEngine+SoftJointLimitSpring")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SoftJointLimitSpring {
    pub m_Spring: f32,
    pub m_Damper: f32,
}
#[cfg(feature = "UnityEngine+SoftJointLimitSpring")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SoftJointLimitSpring =>
    "UnityEngine"."SoftJointLimitSpring"
);
#[cfg(feature = "UnityEngine+SoftJointLimitSpring")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::SoftJointLimitSpring {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SoftJointLimitSpring")]
impl crate::UnityEngine::SoftJointLimitSpring {
    pub fn get_damper(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_damper",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spring(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_spring",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_damper(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_damper",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_spring(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_spring",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
