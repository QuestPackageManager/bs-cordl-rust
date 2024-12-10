#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LinearColor {
    pub m_red: f32,
    pub m_green: f32,
    pub m_blue: f32,
    pub m_intensity: f32,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::LinearColor =>
    "UnityEngine.Experimental.GlobalIllumination"."LinearColor"
);
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
impl crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
    pub fn get_blue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_blue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_green(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_green",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_red(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_red",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_blue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_blue",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_green(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_green",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_red(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_red",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
