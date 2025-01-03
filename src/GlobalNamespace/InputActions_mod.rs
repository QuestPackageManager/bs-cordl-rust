#[cfg(feature = "InputActions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputActions {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _keyboardManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::UIKeyboardManager,
    >,
}
#[cfg(feature = "InputActions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::InputActions => ""
    ."InputActions"
);
#[cfg(feature = "InputActions")]
impl std::ops::Deref for crate::GlobalNamespace::InputActions {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "InputActions")]
impl std::ops::DerefMut for crate::GlobalNamespace::InputActions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "InputActions")]
impl crate::GlobalNamespace::InputActions {
    pub fn HasInputFocus(
        eventSystem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::EventSystem,
        >,
        keyboardManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::UIKeyboardManager,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasInputFocus", (eventSystem, keyboardManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveGameplayCoreSceneContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveGameplayCoreSceneContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToggleAutopilot() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToggleAutopilot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToggleLevelFreeze() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToggleLevelFreeze", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "InputActions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::InputActions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
