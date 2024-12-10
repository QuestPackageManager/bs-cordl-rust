#[cfg(feature = "TMPro+MaterialReference")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MaterialReference {
    pub index: i32,
    pub fontAsset: *mut crate::TMPro::TMP_FontAsset,
    pub spriteAsset: *mut crate::TMPro::TMP_SpriteAsset,
    pub material: *mut crate::UnityEngine::Material,
    pub isDefaultMaterial: bool,
    pub isFallbackMaterial: bool,
    pub fallbackMaterial: *mut crate::UnityEngine::Material,
    pub padding: f32,
    pub referenceCount: i32,
}
#[cfg(feature = "TMPro+MaterialReference")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::MaterialReference => "TMPro"
    ."MaterialReference"
);
#[cfg(feature = "TMPro+MaterialReference")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::MaterialReference {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+MaterialReference")]
impl crate::TMPro::MaterialReference {
    pub fn _ctor(
        &mut self,
        index: i32,
        fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        padding: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (index, fontAsset, spriteAsset, material, padding),
        )?;
        Ok(__cordl_ret.into())
    }
}
