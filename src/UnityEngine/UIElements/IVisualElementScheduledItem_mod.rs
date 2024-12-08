#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduledItem")]
#[repr(C)]
#[derive(Debug)]
pub struct IVisualElementScheduledItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduledItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IVisualElementScheduledItem => "UnityEngine.UIElements"
    ."IVisualElementScheduledItem"
);
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduledItem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IVisualElementScheduledItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduledItem")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IVisualElementScheduledItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduledItem")]
impl crate::UnityEngine::UIElements::IVisualElementScheduledItem {
    pub fn Resume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resume", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartingIn(
        &mut self,
        delayMs: i64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem = __cordl_object
            .invoke("StartingIn", (delayMs))?;
        Ok(__cordl_ret)
    }
    pub fn Every(
        &mut self,
        intervalMs: i64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem = __cordl_object
            .invoke("Every", (intervalMs))?;
        Ok(__cordl_ret)
    }
    pub fn Pause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteLater(
        &mut self,
        delayMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteLater", (delayMs))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduledItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IVisualElementScheduledItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
