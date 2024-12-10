#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct IMGUIContainer {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub m_OnGUIHandler: *mut crate::System::Action,
    pub m_ObjectGUIState: *mut crate::UnityEngine::ObjectGUIState,
    pub useOwnerObjectGUIState: bool,
    pub _lastWorldClip_k__BackingField: crate::UnityEngine::Rect,
    pub m_CullingEnabled: bool,
    pub m_IsFocusDelegated: bool,
    pub m_RefreshCachedLayout: bool,
    pub m_Cache: *mut crate::UnityEngine::GUILayoutUtility_LayoutCache,
    pub m_CachedClippingRect: crate::UnityEngine::Rect,
    pub m_CachedTransform: crate::UnityEngine::Matrix4x4,
    pub _contextType_k__BackingField: crate::UnityEngine::UIElements::ContextType,
    pub lostFocus: bool,
    pub receivedFocus: bool,
    pub focusChangeDirection: *mut crate::UnityEngine::UIElements::FocusChangeDirection,
    pub hasFocusableControls: bool,
    pub newKeyboardFocusControlID: i32,
    pub _focusOnlyIfHasFocusableControls_k__BackingField: bool,
    pub m_GUIGlobals: crate::UnityEngine::UIElements::IMGUIContainer_GUIGlobals,
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IMGUIContainer =>
    "UnityEngine.UIElements"."IMGUIContainer"
);
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IMGUIContainer {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IMGUIContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer")]
impl crate::UnityEngine::UIElements::IMGUIContainer {
    #[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+GUIGlobals")]
    pub type GUIGlobals = crate::UnityEngine::UIElements::IMGUIContainer_GUIGlobals;
    #[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::IMGUIContainer_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::IMGUIContainer_UxmlTraits;
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposeManaged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposeManaged))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoIMGUIRepaint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoIMGUIRepaint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoMeasure(
        &mut self,
        desiredWidth: f32,
        widthMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
        desiredHeight: f32,
        heightMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("DoMeasure", (desiredWidth, widthMode, desiredHeight, heightMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoOnGUI(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        parentTransform: crate::UnityEngine::Matrix4x4,
        clippingRect: crate::UnityEngine::Rect,
        isComputingLayout: bool,
        layoutSize: crate::UnityEngine::Rect,
        onGUIHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
        canAffectFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DoOnGUI",
                (
                    evt,
                    parentTransform,
                    clippingRect,
                    isComputingLayout,
                    layoutSize,
                    onGUIHandler,
                    canAffectFocus,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDefaultAction(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultAction", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentClipRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetCurrentClipRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleIMGUIEvent_Action__cordl_bool1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        onGUIHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
        canAffectFocus: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HandleIMGUIEvent", (e, onGUIHandler, canAffectFocus))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleIMGUIEvent_Matrix4x4_Rect_Action__cordl_bool2(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        worldTransform: crate::UnityEngine::Matrix4x4,
        clippingRect: crate::UnityEngine::Rect,
        onGUIHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
        canAffectFocus: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "HandleIMGUIEvent",
                (e, worldTransform, clippingRect, onGUIHandler, canAffectFocus),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleIMGUIEvent__cordl_bool0(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        canAffectFocus: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HandleIMGUIEvent", (e, canAffectFocus))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsContainerCapturingTheMouse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsContainerCapturingTheMouse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEventInsideLocalWindow(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEventInsideLocalWindow", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLocalEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLocalEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkDirtyLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDirtyLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Action1(
        onGUIHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (onGUIHandler))?;
        Ok(__cordl_object.into())
    }
    pub fn OnGenerateVisualContent(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGenerateVisualContent", (mgc))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn RestoreGlobals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestoreGlobals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveGlobals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveGlobals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEventToIMGUI(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        canAffectFocus: bool,
        verifyBounds: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendEventToIMGUI", (evt, canAffectFocus, verifyBounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEventToIMGUIRaw(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        canAffectFocus: bool,
        verifyBounds: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendEventToIMGUIRaw", (evt, canAffectFocus, verifyBounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFoldoutDepthClass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFoldoutDepthClass", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyBounds(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("VerifyBounds", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn _DoOnGUI_b__56_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DoOnGUI>b__56_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Action1(
        &mut self,
        onGUIHandler: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (onGUIHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutUtility_LayoutCache>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        > = __cordl_object.invoke("get_cache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canGrabFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canGrabFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contextType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::ContextType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ContextType = __cordl_object
            .invoke("get_contextType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cullingEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_cullingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusOnlyIfHasFocusableControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_focusOnlyIfHasFocusableControls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_guiState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ObjectGUIState>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ObjectGUIState> = __cordl_object
            .invoke("get_guiState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastWorldClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_lastWorldClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layoutMeasuredHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_layoutMeasuredHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layoutMeasuredWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_layoutMeasuredWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onGUIHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = __cordl_object
            .invoke("get_onGUIHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_contextType(
        &mut self,
        value: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contextType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lastWorldClip(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lastWorldClip", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_onGUIHandler(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onGUIHandler", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IMGUIContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+GUIGlobals")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IMGUIContainer_GUIGlobals {
    pub matrix: crate::UnityEngine::Matrix4x4,
    pub color: crate::UnityEngine::Color,
    pub contentColor: crate::UnityEngine::Color,
    pub backgroundColor: crate::UnityEngine::Color,
    pub enabled: bool,
    pub changed: bool,
    pub displayIndex: i32,
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+GUIGlobals")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IMGUIContainer_GUIGlobals => "UnityEngine.UIElements"
    ."IMGUIContainer/GUIGlobals"
);
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+GUIGlobals")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::IMGUIContainer_GUIGlobals {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+GUIGlobals")]
impl crate::UnityEngine::UIElements::IMGUIContainer_GUIGlobals {}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct IMGUIContainer_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::IMGUIContainer,
        *mut crate::UnityEngine::UIElements::IMGUIContainer_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IMGUIContainer_UxmlFactory => "UnityEngine.UIElements"
    ."IMGUIContainer/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IMGUIContainer_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::IMGUIContainer,
        *mut crate::UnityEngine::UIElements::IMGUIContainer_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IMGUIContainer_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlFactory")]
impl crate::UnityEngine::UIElements::IMGUIContainer_UxmlFactory {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IMGUIContainer_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct IMGUIContainer_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IMGUIContainer_UxmlTraits => "UnityEngine.UIElements"
    ."IMGUIContainer/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IMGUIContainer_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::VisualElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IMGUIContainer_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlTraits")]
impl crate::UnityEngine::UIElements::IMGUIContainer_UxmlTraits {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+IMGUIContainer+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IMGUIContainer_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
