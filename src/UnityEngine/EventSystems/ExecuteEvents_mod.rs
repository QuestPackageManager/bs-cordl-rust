#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct ExecuteEvents {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::ExecuteEvents =>
    "UnityEngine.EventSystems"."ExecuteEvents"
);
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::ExecuteEvents {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::ExecuteEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl crate::UnityEngine::EventSystems::ExecuteEvents {
    #[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
    pub type EventFunction_1<T1: quest_hook::libil2cpp::Type> = crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
        T1,
    >;
    pub fn CanHandleEvent<T>(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanHandleEvent", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteHierarchy<T>(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
        callbackFunction: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteHierarchy", (root, eventData, callbackFunction))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc18<T>(
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
        functor: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (target, eventData, functor))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc0(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IPointerMoveHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc1(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IPointerEnterHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc10(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IDropHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc11(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IScrollHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc12(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IUpdateSelectedHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc13(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ISelectHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc14(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IDeselectHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc15(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IMoveHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc16(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ISubmitHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc17(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ICancelHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc2(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IPointerExitHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc3(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IPointerDownHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc4(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IPointerUpHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc5(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IPointerClickHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc6(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IInitializePotentialDragHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc7(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IBeginDragHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc8(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IDragHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Gc_Gc9(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IEndDragHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Execute", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEventChain(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        eventChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEventChain", (root, eventChain))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEventHandler<T>(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEventHandler", (root))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEventList<T>(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IEventSystemHandler,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEventList", (go, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSendToComponent<T>(
        component: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldSendToComponent", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateEventData<T>(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateEventData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beginDragHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IBeginDragHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IBeginDragHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_beginDragHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cancelHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ICancelHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ICancelHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_cancelHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deselectHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDeselectHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDeselectHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_deselectHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dragHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_dragHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dropHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDropHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDropHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_dropHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_endDragHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEndDragHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEndDragHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_endDragHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_initializePotentialDrag() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IInitializePotentialDragHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IInitializePotentialDragHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_initializePotentialDrag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_moveHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IMoveHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IMoveHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_moveHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerClickHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerClickHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerClickHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pointerClickHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerDownHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerDownHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerDownHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pointerDownHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerEnterHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerEnterHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerEnterHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pointerEnterHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerExitHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerExitHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerExitHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pointerExitHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerMoveHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerMoveHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerMoveHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pointerMoveHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerUpHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerUpHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IPointerUpHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pointerUpHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IScrollHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IScrollHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_scrollHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISelectHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISelectHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_selectHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_submitHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_submitHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateSelectedHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IUpdateSelectedHandler,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::EventSystems::IUpdateSelectedHandler,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_updateSelectedHandler", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::ExecuteEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ExecuteEvents_EventFunction_1<T1: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1 < T1 > =>
    "UnityEngine.EventSystems"."ExecuteEvents/EventFunction`1" < T1 >
);
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1> {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1> {
    pub fn BeginInvoke(
        &mut self,
        handler: T1,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    >
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (handler, eventData, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        handler: T1,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (handler, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
