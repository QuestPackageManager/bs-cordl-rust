#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReference")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MaterialReference {
    pub index: i32,
    pub fontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
    pub spriteAsset: *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
    pub material: *mut crate::UnityEngine::Material,
    pub isDefaultMaterial: bool,
    pub isFallbackMaterial: bool,
    pub fallbackMaterial: *mut crate::UnityEngine::Material,
    pub padding: f32,
    pub referenceCount: i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReference")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::MaterialReference
    => "UnityEngine.TextCore.Text"."MaterialReference"
);
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReference")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::MaterialReference {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReference")]
impl crate::UnityEngine::TextCore::Text::MaterialReference {
    pub fn AddMaterialReference_FontAsset0(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        materialReferences: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::Text::MaterialReference,
            >,
        >,
        materialReferenceIndexLookup: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddMaterialReference",
                (material, fontAsset, materialReferences, materialReferenceIndexLookup),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddMaterialReference_SpriteAsset1(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
        materialReferences: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::Text::MaterialReference,
            >,
        >,
        materialReferenceIndexLookup: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddMaterialReference",
                (material, spriteAsset, materialReferences, materialReferenceIndexLookup),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        index: i32,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
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
