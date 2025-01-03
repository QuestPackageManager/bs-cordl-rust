#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
#[repr(C)]
#[derive(Debug)]
pub struct UIEventRegistration {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIEventRegistration =>
    "UnityEngine.UIElements"."UIEventRegistration"
);
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIEventRegistration {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIEventRegistration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
impl crate::UnityEngine::UIElements::UIEventRegistration {
    pub fn CleanupRoots() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CleanupRoots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndContainerGUIFromException(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndContainerGUIFromException", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeCurrentIMGUIContainerDirty() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeCurrentIMGUIContainerDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessEvent(
        instanceID: i32,
        nativeEventPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessEvent", (instanceID, nativeEventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterUIElementSystem(
        utility: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IUIElementsUtility,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterUIElementSystem", (utility))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseCapture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseCapture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TakeCapture() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TakeCapture", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIEventRegistration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
