#[cfg(feature = "TMPro+MaterialReference")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::MaterialReference {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "MaterialReference";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::MaterialReference {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::MaterialReference {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::MaterialReference {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::MaterialReference {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::TMPro::MaterialReference>,
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
    pub fn AddMaterialReference_TMP_SpriteAsset1(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        materialReferences: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::TMPro::MaterialReference>,
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
