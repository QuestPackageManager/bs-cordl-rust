#[cfg(feature = "TMPro+MaterialReference")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MaterialReference {
    pub index: i32,
    pub fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    pub spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub isDefaultMaterial: bool,
    pub isFallbackMaterial: bool,
    pub fallbackMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
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
    pub fn AddMaterialReference_TMP_FontAsset0(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        materialReferences: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::MaterialReference>,
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
    pub fn AddMaterialReference_TMP_SpriteAsset1(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        materialReferences: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::MaterialReference>,
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
    pub fn Contains(
        materialReferences: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::TMPro::MaterialReference>,
        >,
        fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (materialReferences, fontAsset))?;
        Ok(__cordl_ret.into())
    }
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
