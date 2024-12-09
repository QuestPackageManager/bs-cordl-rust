#[cfg(feature = "UnityEngine+UIElements+EventDispatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct EventDispatcher {
    __cordl_parent: crate::System::Object,
    pub m_ClickDetector: *mut crate::UnityEngine::UIElements::ClickDetector,
    pub m_DispatchingStrategies: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::IEventDispatchingStrategy,
    >,
    pub m_Queue: *mut crate::System::Collections::Generic::Queue_1<
        crate::UnityEngine::UIElements::EventDispatcher_EventRecord,
    >,
    pub _pointerState_k__BackingField: *mut crate::UnityEngine::UIElements::PointerDispatchState,
    pub m_GateCount: u32,
    pub m_DispatchContexts: *mut crate::System::Collections::Generic::Stack_1<
        crate::UnityEngine::UIElements::EventDispatcher_DispatchContext,
    >,
    pub m_Immediate: bool,
    pub _processingEvents_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventDispatcher =>
    "UnityEngine.UIElements"."EventDispatcher"
);
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventDispatcher {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EventDispatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher")]
impl crate::UnityEngine::UIElements::EventDispatcher {
    #[cfg(feature = "UnityEngine+UIElements+EventDispatcher+DispatchContext")]
    pub type DispatchContext = crate::UnityEngine::UIElements::EventDispatcher_DispatchContext;
    #[cfg(feature = "UnityEngine+UIElements+EventDispatcher+EventRecord")]
    pub type EventRecord = crate::UnityEngine::UIElements::EventDispatcher_EventRecord;
    #[cfg(feature = "UnityEngine+UIElements+EventDispatcher+__c")]
    pub type __c = crate::UnityEngine::UIElements::EventDispatcher___c;
    pub fn ApplyDispatchingStrategies(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
        panel: *mut crate::UnityEngine::UIElements::IPanel,
        imguiEventIsInitiallyUsed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ApplyDispatchingStrategies",
                (evt, panel, imguiEventIsInitiallyUsed),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CloseGate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseGate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispatch(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
        panel: *mut crate::UnityEngine::UIElements::IPanel,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispatch", (evt, panel, dispatchMode))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        strategies: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::UIElements::IEventDispatchingStrategy,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (strategies))?;
        Ok(__cordl_object)
    }
    pub fn OpenGate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenGate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
        panel: *mut crate::UnityEngine::UIElements::IPanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEvent", (evt, panel))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessEventQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEventQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        strategies: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::UIElements::IEventDispatchingStrategy,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (strategies))?;
        Ok(__cordl_ret)
    }
    pub fn get_dispatchImmediately(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_dispatchImmediately", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pointerState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::PointerDispatchState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::PointerDispatchState = __cordl_object
            .invoke("get_pointerState", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_processingEvents(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_processingEvents", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventDispatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher+DispatchContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct EventDispatcher_DispatchContext {
    pub m_GateCount: u32,
    pub m_Queue: *mut crate::System::Collections::Generic::Queue_1<
        crate::UnityEngine::UIElements::EventDispatcher_EventRecord,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher+DispatchContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::EventDispatcher_DispatchContext =>
    "UnityEngine.UIElements"."EventDispatcher/DispatchContext"
);
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher+DispatchContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::EventDispatcher_DispatchContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher+DispatchContext")]
impl crate::UnityEngine::UIElements::EventDispatcher_DispatchContext {}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher+EventRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct EventDispatcher_EventRecord {
    pub m_Event: *mut crate::UnityEngine::UIElements::EventBase,
    pub m_Panel: *mut crate::UnityEngine::UIElements::IPanel,
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher+EventRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::EventDispatcher_EventRecord => "UnityEngine.UIElements"
    ."EventDispatcher/EventRecord"
);
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher+EventRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::EventDispatcher_EventRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatcher+EventRecord")]
impl crate::UnityEngine::UIElements::EventDispatcher_EventRecord {}
