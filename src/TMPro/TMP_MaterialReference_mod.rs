#[cfg(feature = "TMPro+TMP_MaterialReference")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TMP_MaterialReference {
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub referenceCount: i32,
}
#[cfg(feature = "TMPro+TMP_MaterialReference")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_MaterialReference => "TMPro"
    ."TMP_MaterialReference"
);
#[cfg(feature = "TMPro+TMP_MaterialReference")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_MaterialReference {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialReference")]
impl crate::TMPro::TMP_MaterialReference {}
