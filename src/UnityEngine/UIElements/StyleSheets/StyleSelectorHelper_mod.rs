#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSelectorHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper =>
    "UnityEngine.UIElements.StyleSheets"."StyleSelectorHelper"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
