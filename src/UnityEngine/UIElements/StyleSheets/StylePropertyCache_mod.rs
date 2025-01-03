#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyCache")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StylePropertyCache =>
    "UnityEngine.UIElements.StyleSheets"."StylePropertyCache"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyCache")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyCache")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyCache")]
impl crate::UnityEngine::UIElements::StyleSheets::StylePropertyCache {
    pub fn TryGetNonTerminalValue(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syntax: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetNonTerminalValue", (name, syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSyntax(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syntax: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetSyntax", (name, syntax))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
