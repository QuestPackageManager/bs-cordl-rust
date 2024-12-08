#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIElementsUtility =>
    "UnityEngine.UIElements"."UIElementsUtility"
);
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIElementsUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIElementsUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl crate::UnityEngine::UIElements::UIElementsUtility {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_CleanupRoots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.IUIElementsUtility.CleanupRoots", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_EndContainerGUIFromException(
        &mut self,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IUIElementsUtility.EndContainerGUIFromException",
                (exception),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_MakeCurrentIMGUIContainerDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IUIElementsUtility.MakeCurrentIMGUIContainerDirty",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_ProcessEvent(
        &mut self,
        instanceID: i32,
        nativeEventPtr: crate::System::IntPtr,
        eventHandled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IUIElementsUtility.ProcessEvent",
                (instanceID, nativeEventPtr, eventHandled),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_ReleaseCapture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.IUIElementsUtility.ReleaseCapture", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_TakeCapture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.IUIElementsUtility.TakeCapture", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}