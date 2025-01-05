#[cfg(feature = "UnityEngine+TextCore+GlyphMetrics")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GlyphMetrics {
    pub m_Width: f32,
    pub m_Height: f32,
    pub m_HorizontalBearingX: f32,
    pub m_HorizontalBearingY: f32,
    pub m_HorizontalAdvance: f32,
}
#[cfg(feature = "UnityEngine+TextCore+GlyphMetrics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::GlyphMetrics =>
    "UnityEngine.TextCore"."GlyphMetrics"
);
#[cfg(feature = "UnityEngine+TextCore+GlyphMetrics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::GlyphMetrics {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+GlyphMetrics")]
impl crate::UnityEngine::TextCore::GlyphMetrics {
    pub fn Equals_Gc0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_GlyphMetrics1(
        &mut self,
        other: crate::UnityEngine::TextCore::GlyphMetrics,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        width: f32,
        height: f32,
        bearingX: f32,
        bearingY: f32,
        advance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (width, height, bearingX, bearingY, advance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_height(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_height",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalAdvance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_horizontalAdvance",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalBearingX(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_horizontalBearingX",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalBearingY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_horizontalBearingY",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_width",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+GlyphMetrics")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::GlyphMetrics>>
for crate::UnityEngine::TextCore::GlyphMetrics {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::GlyphMetrics> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+TextCore+GlyphMetrics")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::GlyphMetrics>>
for crate::UnityEngine::TextCore::GlyphMetrics {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::GlyphMetrics> {
        todo!()
    }
}
