#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
#[repr(C)]
#[derive(Debug)]
pub struct EventTrigger {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_Delegates: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::EventTrigger_Entry,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::EventTrigger {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "EventTrigger";
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
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::EventTrigger {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::EventTrigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl crate::UnityEngine::EventSystems::EventTrigger {
    #[cfg(feature = "UnityEngine+EventSystems+EventTrigger+Entry")]
    pub type Entry = crate::UnityEngine::EventSystems::EventTrigger_Entry;
    #[cfg(feature = "UnityEngine+EventSystems+EventTrigger+TriggerEvent")]
    pub type TriggerEvent = crate::UnityEngine::EventSystems::EventTrigger_TriggerEvent;
    pub fn Execute(
        &mut self,
        id: crate::UnityEngine::EventSystems::EventTriggerType,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::EventSystems::EventTriggerType,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::BaseEventData,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Execute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Execute", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (id, eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeginDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnBeginDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnBeginDrag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnCancel(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::BaseEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnCancel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnCancel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDeselect(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::BaseEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnDeselect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDeselect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDrag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDrop(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnDrop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDrop", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEndDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnEndDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnEndDrag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnInitializePotentialDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnInitializePotentialDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnInitializePotentialDrag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnMove(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::AxisEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::AxisEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnMove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnMove", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerClick(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPointerClick")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnPointerClick", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerDown(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPointerDown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnPointerDown", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerEnter(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPointerEnter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnPointerEnter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerExit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPointerExit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnPointerExit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerUp(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPointerUp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnPointerUp", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnScroll(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnScroll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnScroll", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnSelect(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::BaseEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnSelect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnSelect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnSubmit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::BaseEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnSubmit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnSubmit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnUpdateSelected(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::BaseEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnUpdateSelected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnUpdateSelected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventData))
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
    pub fn get_delegates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::EventTrigger_Entry,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::EventTrigger_Entry,
                        >,
                    >,
                >,
                0usize,
            >("get_delegates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_delegates", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::EventTrigger_Entry,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_triggers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::EventTrigger_Entry,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::EventTrigger_Entry,
                        >,
                    >,
                >,
                0usize,
            >("get_triggers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_triggers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::EventTrigger_Entry,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_delegates(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::EventTrigger_Entry,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::EventTrigger_Entry,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_delegates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_delegates", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_triggers(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::EventTrigger_Entry,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::EventTrigger_Entry,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_triggers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_triggers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IBeginDragHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IBeginDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IBeginDragHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IBeginDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::ICancelHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::ICancelHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::ICancelHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::ICancelHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IDeselectHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IDeselectHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IDeselectHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IDeselectHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IDragHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IDragHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IDropHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IDropHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IDropHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IDropHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IEndDragHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEndDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IEndDragHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEndDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IInitializePotentialDragHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::EventSystems::IInitializePotentialDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IInitializePotentialDragHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::EventSystems::IInitializePotentialDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IMoveHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IMoveHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IMoveHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IMoveHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerClickHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerClickHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerClickHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerClickHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerDownHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerDownHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerDownHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerDownHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IScrollHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IScrollHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IScrollHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IScrollHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::ISelectHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::ISelectHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::ISelectHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::ISelectHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::ISubmitHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::ISubmitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::ISubmitHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::ISubmitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsRef<crate::UnityEngine::EventSystems::IUpdateSelectedHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IUpdateSelectedHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger")]
impl AsMut<crate::UnityEngine::EventSystems::IUpdateSelectedHandler>
for crate::UnityEngine::EventSystems::EventTrigger {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::EventSystems::IUpdateSelectedHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+Entry")]
#[repr(C)]
#[derive(Debug)]
pub struct EventTrigger_Entry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub eventID: crate::UnityEngine::EventSystems::EventTriggerType,
    pub callback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::EventTrigger_TriggerEvent,
    >,
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+Entry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::EventTrigger_Entry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "EventTrigger/Entry";
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
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+Entry")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::EventTrigger_Entry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+Entry")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::EventTrigger_Entry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+Entry")]
impl crate::UnityEngine::EventSystems::EventTrigger_Entry {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+Entry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::EventTrigger_Entry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+TriggerEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct EventTrigger_TriggerEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    >,
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+TriggerEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::EventTrigger_TriggerEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "EventTrigger/TriggerEvent";
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
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+TriggerEvent")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::EventTrigger_TriggerEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+TriggerEvent")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::EventTrigger_TriggerEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+TriggerEvent")]
impl crate::UnityEngine::EventSystems::EventTrigger_TriggerEvent {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "UnityEngine+EventSystems+EventTrigger+TriggerEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::EventTrigger_TriggerEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
