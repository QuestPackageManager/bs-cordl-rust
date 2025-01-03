#[cfg(feature = "UnityEngine+Touch")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Touch {
    pub m_FingerId: i32,
    pub m_Position: crate::UnityEngine::Vector2,
    pub m_RawPosition: crate::UnityEngine::Vector2,
    pub m_PositionDelta: crate::UnityEngine::Vector2,
    pub m_TimeDelta: f32,
    pub m_TapCount: i32,
    pub m_Phase: crate::UnityEngine::TouchPhase,
    pub m_Type: crate::UnityEngine::TouchType,
    pub m_Pressure: f32,
    pub m_maximumPossiblePressure: f32,
    pub m_Radius: f32,
    pub m_RadiusVariance: f32,
    pub m_AltitudeAngle: f32,
    pub m_AzimuthAngle: f32,
}
#[cfg(feature = "UnityEngine+Touch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Touch => "UnityEngine"."Touch"
);
#[cfg(feature = "UnityEngine+Touch")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Touch {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Touch")]
impl crate::UnityEngine::Touch {
    pub fn get_altitudeAngle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_altitudeAngle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_azimuthAngle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_azimuthAngle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deltaPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deltaTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fingerId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_fingerId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maximumPossiblePressure(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maximumPossiblePressure",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TouchPhase> {
        let __cordl_ret: crate::UnityEngine::TouchPhase = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_phase",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_position",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressure(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pressure",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_radius(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_radius",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_radiusVariance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_radiusVariance",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rawPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rawPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tapCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_tapCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TouchType> {
        let __cordl_ret: crate::UnityEngine::TouchType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_type",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deltaPosition(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_deltaPosition",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_position",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rawPosition(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rawPosition",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
