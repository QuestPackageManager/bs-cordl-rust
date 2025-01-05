#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TextResourceManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextResourceManager
    => "UnityEngine.TextCore.Text"."TextResourceManager"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextResourceManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextResourceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
impl crate::UnityEngine::TextCore::Text::TextResourceManager {
    #[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
    pub type FontAssetRef = crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef;
    pub fn AddFontAsset(
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddFontAsset", (fontAsset))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextResourceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TextResourceManager_FontAssetRef {
    pub nameHashCode: i32,
    pub familyNameHashCode: i32,
    pub styleNameHashCode: i32,
    pub familyNameAndStyleHashCode: i64,
    pub fontAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef =>
    "UnityEngine.TextCore.Text"."TextResourceManager/FontAssetRef"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
impl crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef {
    pub fn _ctor(
        &mut self,
        nameHashCode: i32,
        familyNameHashCode: i32,
        styleNameHashCode: i32,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (nameHashCode, familyNameHashCode, styleNameHashCode, fontAsset),
        )?;
        Ok(__cordl_ret.into())
    }
}
