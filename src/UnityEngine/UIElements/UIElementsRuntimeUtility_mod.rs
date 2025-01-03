#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsRuntimeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIElementsRuntimeUtility => "UnityEngine.UIElements"
    ."UIElementsRuntimeUtility"
);
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIElementsRuntimeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIElementsRuntimeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
impl crate::UnityEngine::UIElements::UIElementsRuntimeUtility {
    #[cfg(
        feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
    )]
    pub type CreateRuntimePanelDelegate = crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate;
    pub fn BeginRenderOverlays(
        displayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginRenderOverlays", (displayIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEvent(
        systemEvent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEvent", (systemEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisposeRuntimePanel(
        ownerObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisposeRuntimePanel", (ownerObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndRenderOverlays(
        displayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndRenderOverlays", (displayIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindOrCreateRuntimePanel(
        ownerObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        createDelegate: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseRuntimePanel>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseRuntimePanel,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindOrCreateRuntimePanel", (ownerObject, createDelegate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSortedPlayerPanels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::Panel,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::Panel,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSortedPlayerPanels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkPotentiallyEmpty(
        settings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PanelSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MarkPotentiallyEmpty", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiDisplayBottomLeftToPanelPosition(
        position: crate::UnityEngine::Vector2,
        targetDisplay: quest_hook::libil2cpp::ByRefMut<crate::System::Nullable_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiDisplayBottomLeftToPanelPosition", (position, targetDisplay))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiDisplayToLocalScreenPosition(
        position: crate::UnityEngine::Vector2,
        targetDisplay: quest_hook::libil2cpp::ByRefMut<crate::System::Nullable_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiDisplayToLocalScreenPosition", (position, targetDisplay))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCachedPanelInternal(
        instanceID: i32,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterCachedPanelInternal", (instanceID, panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterEventSystem(
        eventSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterEventSystem", (eventSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterPlayerloopCallback() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterPlayerloopCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCachedPanelInternal(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveCachedPanelInternal", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveUnusedPanels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveUnusedPanels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderOverlaysBeforePriority(
        displayIndex: i32,
        maxPriority: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RenderOverlaysBeforePriority", (displayIndex, maxPriority))?;
        Ok(__cordl_ret.into())
    }
    pub fn RepaintOffscreenPanels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RepaintOffscreenPanels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RepaintOverlayPanel(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseRuntimePanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RepaintOverlayPanel", (panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenBottomLeftToPanelDelta(
        delta: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScreenBottomLeftToPanelDelta", (delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenBottomLeftToPanelPosition(
        position: crate::UnityEngine::Vector2,
        targetDisplay: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScreenBottomLeftToPanelPosition", (position, targetDisplay))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPanelOrderingDirty() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPanelOrderingDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SortPanels() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortPanels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterEventSystem(
        eventSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterEventSystem", (eventSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterPlayerloopCallback() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterPlayerloopCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRuntimePanels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateRuntimePanels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onCreatePanel(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::BaseRuntimePanel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onCreatePanel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeEventSystem() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_activeEventSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultEventSystem() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DefaultEventSystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DefaultEventSystem,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultEventSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useDefaultEventSystem() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_useDefaultEventSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onCreatePanel(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::BaseRuntimePanel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onCreatePanel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_activeEventSystem(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_activeEventSystem", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsRuntimeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate =>
    "UnityEngine.UIElements"."UIElementsRuntimeUtility/CreateRuntimePanelDelegate"
);
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
impl crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    pub fn Invoke(
        &mut self,
        ownerObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseRuntimePanel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseRuntimePanel,
        > = __cordl_object.invoke("Invoke", (ownerObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIElementsRuntimeUtility+CreateRuntimePanelDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsRuntimeUtility_CreateRuntimePanelDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
