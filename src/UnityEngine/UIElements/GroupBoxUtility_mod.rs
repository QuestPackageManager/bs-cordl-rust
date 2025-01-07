#[cfg(feature = "UnityEngine+UIElements+GroupBoxUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupBoxUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+GroupBoxUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::GroupBoxUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "GroupBoxUtility";
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
#[cfg(feature = "UnityEngine+UIElements+GroupBoxUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::GroupBoxUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBoxUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::GroupBoxUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBoxUtility")]
impl crate::UnityEngine::UIElements::GroupBoxUtility {
    pub fn FindOrCreateGroupManager(
        groupBox: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IGroupBox>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IGroupManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupManager,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindOrCreateGroupManager", (groupBox))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnGroupBoxDetachedFromPanel(
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DetachFromPanelEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnGroupBoxDetachedFromPanel", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnOptionSelected<T>(
        selectedOption: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnOptionSelected", (selectedOption))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPanelDestroyed(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnPanelDestroyed", (panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterGroupBoxOption<T>(
        option: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterGroupBoxOption", (option))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterGroupBoxOption<T>(
        option: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterGroupBoxOption", (option))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBoxUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::GroupBoxUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
