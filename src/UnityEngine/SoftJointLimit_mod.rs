#[cfg(feature = "UnityEngine+SoftJointLimit")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SoftJointLimit {
    pub m_Limit: f32,
    pub m_Bounciness: f32,
    pub m_ContactDistance: f32,
}
#[cfg(feature = "UnityEngine+SoftJointLimit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SoftJointLimit => "UnityEngine"
    ."SoftJointLimit"
);
#[cfg(feature = "UnityEngine+SoftJointLimit")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::SoftJointLimit {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SoftJointLimit")]
impl crate::UnityEngine::SoftJointLimit {
    pub fn get_limit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_limit",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_limit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_limit",
            (value),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_spring(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_spring",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_bouncyness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bouncyness",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_contactDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_contactDistance",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_bounciness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bounciness",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_damper(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_damper",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_bouncyness(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bouncyness",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
