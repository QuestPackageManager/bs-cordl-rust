#[cfg(feature = "UnityEngine+WheelFrictionCurve")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WheelFrictionCurve {
    pub m_ExtremumSlip: f32,
    pub m_ExtremumValue: f32,
    pub m_AsymptoteSlip: f32,
    pub m_AsymptoteValue: f32,
    pub m_Stiffness: f32,
}
#[cfg(feature = "UnityEngine+WheelFrictionCurve")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::WheelFrictionCurve => "UnityEngine"
    ."WheelFrictionCurve"
);
#[cfg(feature = "UnityEngine+WheelFrictionCurve")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::WheelFrictionCurve {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+WheelFrictionCurve")]
impl crate::UnityEngine::WheelFrictionCurve {
    pub fn get_asymptoteSlip(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_asymptoteSlip",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_asymptoteValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_asymptoteValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_extremumSlip(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_extremumSlip",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_extremumValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_extremumValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_stiffness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_stiffness",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_asymptoteSlip(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_asymptoteSlip",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_asymptoteValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_asymptoteValue",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_extremumSlip(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_extremumSlip",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_extremumValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_extremumValue",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_stiffness(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_stiffness",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
