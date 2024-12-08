#[cfg(feature = "UnityEngine+JointMotor")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JointMotor {
    pub m_TargetVelocity: f32,
    pub m_Force: f32,
    pub m_FreeSpin: i32,
}
#[cfg(feature = "UnityEngine+JointMotor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::JointMotor => "UnityEngine"
    ."JointMotor"
);
#[cfg(feature = "UnityEngine+JointMotor")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::JointMotor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+JointMotor")]
impl crate::UnityEngine::JointMotor {
    pub fn set_targetVelocity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_targetVelocity",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_targetVelocity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_targetVelocity",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_freeSpin(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_freeSpin",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_force(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_force",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_freeSpin(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_freeSpin",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_force(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_force",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
