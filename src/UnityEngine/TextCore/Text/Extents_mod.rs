#[cfg(feature = "UnityEngine+TextCore+Text+Extents")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Extents {
    pub min: crate::UnityEngine::Vector2,
    pub max: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+TextCore+Text+Extents")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::Extents =>
    "UnityEngine.TextCore.Text"."Extents"
);
#[cfg(feature = "UnityEngine+TextCore+Text+Extents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::Extents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+Extents")]
impl crate::UnityEngine::TextCore::Text::Extents {
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
}
