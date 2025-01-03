#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GlyphValueRecord_Legacy {
    pub xPlacement: f32,
    pub yPlacement: f32,
    pub xAdvance: f32,
    pub yAdvance: f32,
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::GlyphValueRecord_Legacy => "TMPro"
    ."GlyphValueRecord_Legacy"
);
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::GlyphValueRecord_Legacy {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
impl crate::TMPro::GlyphValueRecord_Legacy {
    pub fn _ctor(
        &mut self,
        valueRecord: crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (valueRecord),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: crate::TMPro::GlyphValueRecord_Legacy,
        b: crate::TMPro::GlyphValueRecord_Legacy,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::GlyphValueRecord_Legacy> {
        let __cordl_ret: crate::TMPro::GlyphValueRecord_Legacy = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
