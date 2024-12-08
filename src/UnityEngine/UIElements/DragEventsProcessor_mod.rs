#[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct DragEventsProcessor {
    __cordl_parent: crate::System::Object,
    pub m_IsRegistered: bool,
    pub m_DragState: crate::UnityEngine::UIElements::DragEventsProcessor_DragState,
    pub m_Start: crate::UnityEngine::Vector3,
    pub m_Target: *mut crate::UnityEngine::UIElements::VisualElement,
}
#[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DragEventsProcessor =>
    "UnityEngine.UIElements"."DragEventsProcessor"
);
#[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DragEventsProcessor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DragEventsProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor")]
impl crate::UnityEngine::UIElements::DragEventsProcessor {
    #[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor+DragState")]
    pub type DragState = crate::UnityEngine::UIElements::DragEventsProcessor_DragState;
    pub fn CanStartDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanStartDrag", (pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterCallbacksFromTarget_DetachFromPanelEvent0(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::DetachFromPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallbacksFromTarget", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterCallbacksFromTarget__cordl_bool1(
        &mut self,
        unregisterPanelEvents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallbacksFromTarget", (unregisterPanelEvents))?;
        Ok(__cordl_ret)
    }
    pub fn get_supportsDragEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_supportsDragEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerLeaveEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerLeaveEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerLeaveEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnDrop(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrop", (pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbacksFromTarget_AttachToPanelEvent0(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::AttachToPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbacksFromTarget", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbacksFromTarget_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbacksFromTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUpEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerUpEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUpEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDownEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerDownEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDownEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn get_isEditorContext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEditorContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useDragEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useDragEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dragAndDrop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IDragAndDrop,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IDragAndDrop = __cordl_object
            .invoke("get_dragAndDrop", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        target: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (target))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerCancelEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerCancelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerCancelEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerCapturedOut(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerCaptureOutEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerCapturedOut", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerMoveEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerMoveEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMoveEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn GetDropTarget(
        &mut self,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::DragEventsProcessor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::DragEventsProcessor = __cordl_object
            .invoke("GetDropTarget", (position))?;
        Ok(__cordl_ret)
    }
    pub fn StartDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StartDragArgs> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StartDragArgs = __cordl_object
            .invoke("StartDrag", (pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDrag", (pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDragAndDropUI(
        &mut self,
        dragCancelled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDragAndDropUI", (dragCancelled))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        target: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (target))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DragEventsProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor+DragState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragEventsProcessor_DragState {
    CanStartDrag = 1i32,
    Dragging = 2i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+DragEventsProcessor+DragState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DragEventsProcessor_DragState => "UnityEngine.UIElements"
    ."DragEventsProcessor/DragState"
);
