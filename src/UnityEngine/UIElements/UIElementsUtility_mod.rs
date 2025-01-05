#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIElementsUtility =>
    "UnityEngine.UIElements"."UIElementsUtility"
);
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIElementsUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIElementsUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl crate::UnityEngine::UIElements::UIElementsUtility {
    pub fn BeginContainerGUI(
        cache: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        >,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        container: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IMGUIContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginContainerGUI", (cache, evt, container))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEvent_Event0(
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
    pub fn CreateEvent_EventType1(
        systemEvent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        eventType: crate::UnityEngine::EventType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEvent", (systemEvent, eventType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoDispatch(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoDispatch", (panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndContainerGUI(
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        layoutSize: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndContainerGUI", (evt, layoutSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllPanels(
        panels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
            >,
        >,
        contextType: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllPanels", (panels, contextType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPanelsIterator() -> quest_hook::libil2cpp::Result<
        crate::System::Collections::Generic::Dictionary_2_Enumerator<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
        >,
    > {
        let __cordl_ret: crate::System::Collections::Generic::Dictionary_2_Enumerator<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPanelsIterator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseMenuName(
        menuName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseMenuName", (menuName))?;
        Ok(__cordl_ret.into())
    }
    pub fn PixelsPerUnitScaleForElement(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PixelsPerUnitScaleForElement", (ve, sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCachedPanel(
        instanceID: i32,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterCachedPanel", (instanceID, panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCachedPanel(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveCachedPanel", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPanel(
        instanceID: i32,
        panel: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetPanel", (instanceID, panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_CleanupRoots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.IUIElementsUtility.CleanupRoots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_EndContainerGUIFromException(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IUIElementsUtility.EndContainerGUIFromException",
                (exception),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_MakeCurrentIMGUIContainerDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IUIElementsUtility.MakeCurrentIMGUIContainerDirty",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_ProcessEvent(
        &mut self,
        instanceID: i32,
        nativeEventPtr: crate::System::IntPtr,
        eventHandled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IUIElementsUtility.ProcessEvent",
                (instanceID, nativeEventPtr, eventHandled),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_ReleaseCapture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.IUIElementsUtility.ReleaseCapture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_TakeCapture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.IUIElementsUtility.TakeCapture", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl AsRef<crate::UnityEngine::UIElements::IUIElementsUtility>
for crate::UnityEngine::UIElements::UIElementsUtility {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IUIElementsUtility {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl AsMut<crate::UnityEngine::UIElements::IUIElementsUtility>
for crate::UnityEngine::UIElements::UIElementsUtility {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IUIElementsUtility {
        unsafe { std::mem::transmute(self) }
    }
}
