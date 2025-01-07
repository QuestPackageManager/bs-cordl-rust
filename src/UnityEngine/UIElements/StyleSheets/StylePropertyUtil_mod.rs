#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyUtil")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyUtil {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets";
    const CLASS_NAME: &'static str = "StylePropertyUtil";
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
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyUtil")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheets::StylePropertyUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyUtil")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyUtil")]
impl crate::UnityEngine::UIElements::StyleSheets::StylePropertyUtil {
    pub fn IsAnimatable(
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAnimatable", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMatchingShorthand(
        shorthand: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMatchingShorthand", (shorthand, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetEnumIntValue(
        enumType: crate::UnityEngine::UIElements::StyleSheets::StyleEnumType,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        intValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetEnumIntValue", (enumType, value, intValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyUtil")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
