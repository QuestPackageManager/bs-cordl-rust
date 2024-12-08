#[cfg(feature = "UnityEngine+UIElements+IUIElementsUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct IUIElementsUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IUIElementsUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IUIElementsUtility =>
    "UnityEngine.UIElements"."IUIElementsUtility"
);
#[cfg(feature = "UnityEngine+UIElements+IUIElementsUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IUIElementsUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUIElementsUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IUIElementsUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUIElementsUtility")]
impl crate::UnityEngine::UIElements::IUIElementsUtility {
    pub fn CleanupRoots(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CleanupRoots", ())?;
        Ok(__cordl_ret)
    }
    pub fn EndContainerGUIFromException(
        &mut self,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndContainerGUIFromException", (exception))?;
        Ok(__cordl_ret)
    }
    pub fn MakeCurrentIMGUIContainerDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MakeCurrentIMGUIContainerDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessEvent(
        &mut self,
        instanceID: i32,
        nativeEventPtr: crate::System::IntPtr,
        eventHandled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ProcessEvent", (instanceID, nativeEventPtr, eventHandled))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseCapture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseCapture", ())?;
        Ok(__cordl_ret)
    }
    pub fn TakeCapture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TakeCapture", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUIElementsUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IUIElementsUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
