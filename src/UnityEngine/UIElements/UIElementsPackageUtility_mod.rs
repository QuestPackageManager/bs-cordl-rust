#[cfg(feature = "UnityEngine+UIElements+UIElementsPackageUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsPackageUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsPackageUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIElementsPackageUtility => "UnityEngine.UIElements"
    ."UIElementsPackageUtility"
);
#[cfg(feature = "UnityEngine+UIElements+UIElementsPackageUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIElementsPackageUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsPackageUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIElementsPackageUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsPackageUtility")]
impl crate::UnityEngine::UIElements::UIElementsPackageUtility {
    pub fn Refresh() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EditorResourcesBasePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_EditorResourcesBasePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUIEPackageLoaded() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsUIEPackageLoaded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EditorResourcesBasePath(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_EditorResourcesBasePath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsUIEPackageLoaded(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_IsUIEPackageLoaded", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsPackageUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsPackageUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
