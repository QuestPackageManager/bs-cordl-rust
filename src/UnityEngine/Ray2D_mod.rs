#[cfg(feature = "UnityEngine+Ray2D")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ray2D {
    pub m_Origin: crate::UnityEngine::Vector2,
    pub m_Direction: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+Ray2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Ray2D => "UnityEngine"."Ray2D"
);
#[cfg(feature = "UnityEngine+Ray2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Ray2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Ray2D")]
impl crate::UnityEngine::Ray2D {
    pub fn GetPoint(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPoint",
            (distance),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String_IFormatProvider1(
        &mut self,
        format: *mut crate::System::String,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (origin, direction),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_direction",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_origin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_origin",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
