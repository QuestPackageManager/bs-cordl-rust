#[cfg(feature = "UnityEngine+UIElements+PointerDispatchState")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerDispatchState {
    __cordl_parent: crate::System::Object,
    pub m_PendingPointerCapture: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::IEventHandler,
    >,
    pub m_PointerCapture: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::IEventHandler,
    >,
    pub m_ShouldSendCompatibilityMouseEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        bool,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+PointerDispatchState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerDispatchState =>
    "UnityEngine.UIElements"."PointerDispatchState"
);
#[cfg(feature = "UnityEngine+UIElements+PointerDispatchState")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerDispatchState {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerDispatchState")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerDispatchState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerDispatchState")]
impl crate::UnityEngine::UIElements::PointerDispatchState {
    pub fn ActivateCompatibilityMouseEvents(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateCompatibilityMouseEvents", (pointerId))?;
        Ok(__cordl_ret)
    }
    pub fn CapturePointer(
        &mut self,
        handler: *mut crate::UnityEngine::UIElements::IEventHandler,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CapturePointer", (handler, pointerId))?;
        Ok(__cordl_ret)
    }
    pub fn GetCapturingElement(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IEventHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IEventHandler = __cordl_object
            .invoke("GetCapturingElement", (pointerId))?;
        Ok(__cordl_ret)
    }
    pub fn HasPointerCapture(
        &mut self,
        handler: *mut crate::UnityEngine::UIElements::IEventHandler,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasPointerCapture", (handler, pointerId))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PreventCompatibilityMouseEvents(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreventCompatibilityMouseEvents", (pointerId))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPointerCapture(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerCapture", (pointerId))?;
        Ok(__cordl_ret)
    }
    pub fn ReleasePointer_IEventHandler_i32_1(
        &mut self,
        handler: *mut crate::UnityEngine::UIElements::IEventHandler,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleasePointer", (handler, pointerId))?;
        Ok(__cordl_ret)
    }
    pub fn ReleasePointer_i32_0(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleasePointer", (pointerId))?;
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
    pub fn ShouldSendCompatibilityMouseEvents(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::IPointerEvent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldSendCompatibilityMouseEvents", (evt))?;
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
#[cfg(feature = "UnityEngine+UIElements+PointerDispatchState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerDispatchState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}