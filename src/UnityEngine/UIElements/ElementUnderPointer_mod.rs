#[cfg(feature = "UnityEngine+UIElements+ElementUnderPointer")]
#[repr(C)]
#[derive(Debug)]
pub struct ElementUnderPointer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_PendingTopElementUnderPointer: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        >,
    >,
    pub m_TopElementUnderPointer: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        >,
    >,
    pub m_TriggerPointerEvent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPointerEvent>,
        >,
    >,
    pub m_TriggerMouseEvent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IMouseEvent>,
        >,
    >,
    pub m_PickingPointerPositions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    >,
    pub m_IsPickingPointerTemporaries: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<bool>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ElementUnderPointer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ElementUnderPointer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ElementUnderPointer";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::EventDispatcher,
                    >,
                    crate::UnityEngine::UIElements::ContextType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CommitElementUnderPointers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CommitElementUnderPointers", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dispatcher, contextType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEventPointerPosition(
        &mut self,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>),
                crate::UnityEngine::Vector2,
                1usize,
            >("GetEventPointerPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEventPointerPosition", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, (triggerEvent))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                3usize,
            >("GetTopElementUnderPointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTopElementUnderPointer", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe {
            method.invoke_unchecked(self, (pointerId, pickPosition, isTemporary))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTopElementUnderPointer_i32_1(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                1usize,
            >("GetTopElementUnderPointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTopElementUnderPointer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked(self, (pointerId)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetElementUnderPointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetElementUnderPointer", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (newElementUnderPointer, pointerId, triggerEvent),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetElementUnderPointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetElementUnderPointer", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (newElementUnderPointer, pointerId, triggerEvent, temporary),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    i32,
                    crate::UnityEngine::Vector2,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetElementUnderPointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetElementUnderPointer", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (newElementUnderPointer, pointerId, pointerPos))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetTemporaryElementUnderPointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetTemporaryElementUnderPointer", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (newElementUnderPointer, pointerId, triggerEvent),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
