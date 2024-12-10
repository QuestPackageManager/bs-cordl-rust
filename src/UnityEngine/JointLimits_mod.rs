#[cfg(feature = "UnityEngine+JointLimits")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JointLimits {
    pub m_Min: f32,
    pub m_Max: f32,
    pub m_Bounciness: f32,
    pub m_BounceMinVelocity: f32,
    pub m_ContactDistance: f32,
    pub minBounce: f32,
    pub maxBounce: f32,
}
#[cfg(feature = "UnityEngine+JointLimits")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::JointLimits => "UnityEngine"
    ."JointLimits"
);
#[cfg(feature = "UnityEngine+JointLimits")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::JointLimits {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+JointLimits")]
impl crate::UnityEngine::JointLimits {
    pub fn get_bounceMinVelocity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bounceMinVelocity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bounciness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bounciness",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contactDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_contactDistance",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_max(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_max",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_min(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_min",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bounceMinVelocity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bounceMinVelocity",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bounciness(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bounciness",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_contactDistance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_contactDistance",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_max(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_max",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_min(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_min",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
