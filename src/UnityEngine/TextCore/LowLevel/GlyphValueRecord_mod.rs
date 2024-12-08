#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphValueRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlyphValueRecord {
    pub m_XPlacement: f32,
    pub m_YPlacement: f32,
    pub m_XAdvance: f32,
    pub m_YAdvance: f32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphValueRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::GlyphValueRecord =>
    "UnityEngine.TextCore.LowLevel"."GlyphValueRecord"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphValueRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphValueRecord")]
impl crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord {
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_GlyphValueRecord1(
        &mut self,
        other: crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_yPlacement(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yPlacement",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_xAdvance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xAdvance",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_yAdvance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yAdvance",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_yPlacement(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yPlacement",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_xPlacement(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xPlacement",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_xPlacement(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xPlacement",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
