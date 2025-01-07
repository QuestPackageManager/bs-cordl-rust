#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSheetColor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetColor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets";
    const CLASS_NAME: &'static str = "StyleSheetColor";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetColor")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheets::StyleSheetColor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::UnityEngine::UIElements::StyleSheets::StyleSheetColor {
    pub fn HexToColor32(
        color: u32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexToColor32", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetColor(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetColor", (name, color))?;
        Ok(__cordl_ret.into())
    }
}
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
