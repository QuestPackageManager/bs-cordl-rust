#[cfg(feature = "UnityEngine+JointDrive")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct JointDrive {
    pub m_PositionSpring: f32,
    pub m_PositionDamper: f32,
    pub m_MaximumForce: f32,
    pub m_UseAcceleration: i32,
}
#[cfg(feature = "UnityEngine+JointDrive")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::JointDrive => "UnityEngine"
    ."JointDrive"
);
#[cfg(feature = "UnityEngine+JointDrive")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::JointDrive {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+JointDrive")]
impl crate::UnityEngine::JointDrive {
    pub fn get_maximumForce(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maximumForce",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::JointDriveMode> {
        let __cordl_ret: crate::UnityEngine::JointDriveMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_mode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_positionDamper(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_positionDamper",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_positionSpring(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_positionSpring",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useAcceleration(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_useAcceleration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maximumForce(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_maximumForce",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mode(
        &mut self,
        value: crate::UnityEngine::JointDriveMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_mode",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_positionDamper(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_positionDamper",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_positionSpring(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_positionSpring",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useAcceleration(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_useAcceleration",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
