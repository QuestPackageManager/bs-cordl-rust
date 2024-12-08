#[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings+Anchor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoUnwrapSettings_Anchor {
    LowerCenter = 7i32,
    LowerLeft = 6i32,
    LowerRight = 8i32,
    MiddleCenter = 4i32,
    MiddleLeft = 3i32,
    MiddleRight = 5i32,
    None = 9i32,
    UpperCenter = 1i32,
    UpperLeft = 0i32,
    UpperRight = 2i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings+Anchor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::AutoUnwrapSettings_Anchor => "UnityEngine.ProBuilder"
    ."AutoUnwrapSettings/Anchor"
);
#[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AutoUnwrapSettings {
    pub m_UseWorldSpace: bool,
    pub m_FlipU: bool,
    pub m_FlipV: bool,
    pub m_SwapUV: bool,
    pub m_Fill: crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Fill,
    pub m_Scale: crate::UnityEngine::Vector2,
    pub m_Offset: crate::UnityEngine::Vector2,
    pub m_Rotation: f32,
    pub m_Anchor: crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Anchor,
}
#[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::AutoUnwrapSettings =>
    "UnityEngine.ProBuilder"."AutoUnwrapSettings"
);
#[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::AutoUnwrapSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings")]
impl crate::UnityEngine::ProBuilder::AutoUnwrapSettings {
    #[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings+Fill")]
    pub type Fill = crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Fill;
    #[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings+Anchor")]
    pub type Anchor = crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Anchor;
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        unwrapSettings: crate::UnityEngine::ProBuilder::AutoUnwrapSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (unwrapSettings),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_anchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Anchor,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Anchor = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_anchor",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_fill(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Fill,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Fill = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_fill",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_flipU(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flipU",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_flipV(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flipV",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_offset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_offset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_rotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_scale",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_swapUV(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_swapUV",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_useWorldSpace(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_useWorldSpace",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_anchor(
        &mut self,
        value: crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Anchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_anchor",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_fill(
        &mut self,
        value: crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Fill,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_fill",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_flipU(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_flipU",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_flipV(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_flipV",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_offset(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_offset",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_rotation(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rotation",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_scale(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_scale",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_swapUV(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_swapUV",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_useWorldSpace(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_useWorldSpace",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings+Fill")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoUnwrapSettings_Fill {
    Fit = 0i32,
    Stretch = 2i32,
    Tile = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+AutoUnwrapSettings+Fill")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::AutoUnwrapSettings_Fill
    => "UnityEngine.ProBuilder"."AutoUnwrapSettings/Fill"
);
