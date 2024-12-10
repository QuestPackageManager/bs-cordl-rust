#[cfg(feature = "UnityEngine+UIElements+ElementUnderPointer")]
#[repr(C)]
#[derive(Debug)]
pub struct ElementUnderPointer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_PendingTopElementUnderPointer: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_TopElementUnderPointer: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_TriggerPointerEvent: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::IPointerEvent,
    >,
    pub m_TriggerMouseEvent: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::IMouseEvent,
    >,
    pub m_PickingPointerPositions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector2,
    >,
    pub m_IsPickingPointerTemporaries: *mut quest_hook::libil2cpp::Il2CppArray<bool>,
}
#[cfg(feature = "UnityEngine+UIElements+ElementUnderPointer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ElementUnderPointer =>
    "UnityEngine.UIElements"."ElementUnderPointer"
);
#[cfg(feature = "UnityEngine+UIElements+ElementUnderPointer")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ElementUnderPointer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ElementUnderPointer")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ElementUnderPointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ElementUnderPointer")]
impl crate::UnityEngine::UIElements::ElementUnderPointer {
    pub fn CommitElementUnderPointers(
        &mut self,
        dispatcher: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventDispatcher,
        >,
        contextType: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CommitElementUnderPointers", (dispatcher, contextType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEventPointerPosition(
        &mut self,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetEventPointerPosition", (triggerEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTopElementUnderPointer_ByRefMut_ByRefMut0(
        &mut self,
        pointerId: i32,
        pickPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        isTemporary: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object
            .invoke(
                "GetTopElementUnderPointer",
                (pointerId, pickPosition, isTemporary),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTopElementUnderPointer_i32_1(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("GetTopElementUnderPointer", (pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetElementUnderPointer_EventBase1(
        &mut self,
        newElementUnderPointer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        pointerId: i32,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetElementUnderPointer",
                (newElementUnderPointer, pointerId, triggerEvent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetElementUnderPointer_EventBase__cordl_bool2(
        &mut self,
        newElementUnderPointer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        pointerId: i32,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
        temporary: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetElementUnderPointer",
                (newElementUnderPointer, pointerId, triggerEvent, temporary),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetElementUnderPointer_Vector2_0(
        &mut self,
        newElementUnderPointer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        pointerId: i32,
        pointerPos: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetElementUnderPointer",
                (newElementUnderPointer, pointerId, pointerPos),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTemporaryElementUnderPointer(
        &mut self,
        newElementUnderPointer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        pointerId: i32,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetTemporaryElementUnderPointer",
                (newElementUnderPointer, pointerId, triggerEvent),
            )?;
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
#[cfg(feature = "UnityEngine+UIElements+ElementUnderPointer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ElementUnderPointer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
