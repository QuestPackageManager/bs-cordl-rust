#[cfg(feature = "UnityEngine+UIElements+ClickDetector+ButtonClickStatus")]
#[repr(C)]
#[derive(Debug)]
pub struct ClickDetector_ButtonClickStatus {
    __cordl_parent: crate::System::Object,
    pub m_Target: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_PointerDownPosition: crate::UnityEngine::Vector3,
    pub m_LastPointerDownTime: i64,
    pub m_ClickCount: i32,
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector+ButtonClickStatus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ClickDetector_ButtonClickStatus =>
    "UnityEngine.UIElements"."ClickDetector/ButtonClickStatus"
);
#[cfg(feature = "UnityEngine+UIElements+ClickDetector+ButtonClickStatus")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::ClickDetector_ButtonClickStatus {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector+ButtonClickStatus")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::ClickDetector_ButtonClickStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector+ButtonClickStatus")]
impl crate::UnityEngine::UIElements::ClickDetector_ButtonClickStatus {
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
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector+ButtonClickStatus")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ClickDetector_ButtonClickStatus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector")]
#[repr(C)]
#[derive(Debug)]
pub struct ClickDetector {
    __cordl_parent: crate::System::Object,
    pub m_ClickStatus: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::ClickDetector_ButtonClickStatus,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ClickDetector =>
    "UnityEngine.UIElements"."ClickDetector"
);
#[cfg(feature = "UnityEngine+UIElements+ClickDetector")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ClickDetector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ClickDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector")]
impl crate::UnityEngine::UIElements::ClickDetector {
    #[cfg(feature = "UnityEngine+UIElements+ClickDetector+ButtonClickStatus")]
    pub type ButtonClickStatus = crate::UnityEngine::UIElements::ClickDetector_ButtonClickStatus;
    pub fn CancelClickTracking(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelClickTracking", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn Cleanup(
        &mut self,
        elements: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", (elements))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn SendClickEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendClickEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn StartClickTracking(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartClickTracking", (evt))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickDetector")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ClickDetector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
