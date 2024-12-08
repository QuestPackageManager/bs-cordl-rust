#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSheetColor {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StyleSheetColor =>
    "UnityEngine.UIElements.StyleSheets"."StyleSheetColor"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheets::StyleSheetColor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleSheetColor {}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}