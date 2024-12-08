#[cfg(feature = "UnityEngine+InputSystem+XR+Eyes")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Eyes {
    pub m_LeftEyePosition: crate::UnityEngine::Vector3,
    pub m_LeftEyeRotation: crate::UnityEngine::Quaternion,
    pub m_RightEyePosition: crate::UnityEngine::Vector3,
    pub m_RightEyeRotation: crate::UnityEngine::Quaternion,
    pub m_FixationPoint: crate::UnityEngine::Vector3,
    pub m_LeftEyeOpenAmount: f32,
    pub m_RightEyeOpenAmount: f32,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Eyes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::Eyes =>
    "UnityEngine.InputSystem.XR"."Eyes"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+Eyes")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Eyes {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Eyes")]
impl crate::UnityEngine::InputSystem::XR::Eyes {
    pub fn set_leftEyeOpenAmount(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_leftEyeOpenAmount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_rightEyePosition(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rightEyePosition",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_leftEyeOpenAmount(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_leftEyeOpenAmount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_rightEyePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rightEyePosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_fixationPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_fixationPoint",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_rightEyeOpenAmount(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rightEyeOpenAmount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_rightEyeOpenAmount(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rightEyeOpenAmount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_leftEyePosition(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_leftEyePosition",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_leftEyeRotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_leftEyeRotation",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_leftEyePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_leftEyePosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_leftEyeRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_leftEyeRotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_rightEyeRotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rightEyeRotation",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_fixationPoint(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_fixationPoint",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_rightEyeRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rightEyeRotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
