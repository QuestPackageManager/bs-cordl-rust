#[cfg(feature = "TMPro+Extents")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Extents {
    pub min: crate::UnityEngine::Vector2,
    pub max: crate::UnityEngine::Vector2,
}
#[cfg(feature = "TMPro+Extents")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::Extents => "TMPro"."Extents"
);
#[cfg(feature = "TMPro+Extents")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::Extents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+Extents")]
impl crate::TMPro::Extents {
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
        min: crate::UnityEngine::Vector2,
        max: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (min, max),
        )?;
        Ok(__cordl_ret)
    }
}
