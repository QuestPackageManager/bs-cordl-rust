#[cfg(feature = "TMPro+VertexGradient")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VertexGradient {
    pub topLeft: crate::UnityEngine::Color,
    pub topRight: crate::UnityEngine::Color,
    pub bottomLeft: crate::UnityEngine::Color,
    pub bottomRight: crate::UnityEngine::Color,
}
#[cfg(feature = "TMPro+VertexGradient")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::VertexGradient => "TMPro"
    ."VertexGradient"
);
#[cfg(feature = "TMPro+VertexGradient")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::VertexGradient {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+VertexGradient")]
impl crate::TMPro::VertexGradient {
    pub fn _ctor_Color0(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (color),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Color_Color_Color1(
        &mut self,
        color0: crate::UnityEngine::Color,
        color1: crate::UnityEngine::Color,
        color2: crate::UnityEngine::Color,
        color3: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (color0, color1, color2, color3),
        )?;
        Ok(__cordl_ret)
    }
}
