#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsRuntimeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIElementsRuntimeUtility => "UnityEngine.UIElements"
    ."UIElementsRuntimeUtility"
);
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIElementsRuntimeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIElementsRuntimeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
impl crate::UnityEngine::UIElements::UIElementsRuntimeUtility {
    #[cfg(
        feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
    )]
    pub type CreateRuntimePanelDelegate = crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate;
    #[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+__c")]
    pub type __c = crate::UnityEngine::UIElements::UIElementsRuntimeUtility___c;
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsRuntimeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate =>
    "UnityEngine.UIElements"."UIElementsRuntimeUtility/CreateRuntimePanelDelegate"
);
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
impl crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    pub fn Invoke(
        &mut self,
        ownerObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseRuntimePanel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseRuntimePanel,
        > = __cordl_object.invoke("Invoke", (ownerObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
