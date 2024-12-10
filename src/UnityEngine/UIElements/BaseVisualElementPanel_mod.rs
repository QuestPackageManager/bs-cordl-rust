#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseVisualElementPanel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub panelDisposed: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::BaseVisualElementPanel,
    >,
    pub m_UIElementsBridge: *mut crate::UnityEngine::UIElements::UIElementsBridge,
    pub m_Scale: f32,
    pub yogaConfig: *mut crate::UnityEngine::Yoga::YogaConfig,
    pub m_PixelsPerPoint: f32,
    pub _referenceSpritePixelsPerUnit_k__BackingField: f32,
    pub _clearSettings_k__BackingField: crate::UnityEngine::UIElements::PanelClearSettings,
    pub _duringLayoutPhase_k__BackingField: bool,
    pub _repaintData_k__BackingField: *mut crate::UnityEngine::UIElements::RepaintData,
    pub _cursorManager_k__BackingField: *mut crate::UnityEngine::UIElements::ICursorManager,
    pub _contextualMenuManager_k__BackingField: *mut crate::UnityEngine::UIElements::ContextualMenuManager,
    pub _disposed_k__BackingField: bool,
    pub m_TopElementUnderPointers: *mut crate::UnityEngine::UIElements::ElementUnderPointer,
    pub standardShaderChanged: *mut crate::System::Action,
    pub standardWorldSpaceShaderChanged: *mut crate::System::Action,
    pub atlasChanged: *mut crate::System::Action,
    pub updateMaterial: *mut crate::System::Action_1<*mut crate::UnityEngine::Material>,
    pub hierarchyChanged: *mut crate::UnityEngine::UIElements::HierarchyEvent,
    pub beforeUpdate: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::IPanel,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseVisualElementPanel
    => "UnityEngine.UIElements"."BaseVisualElementPanel"
);
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl crate::UnityEngine::UIElements::BaseVisualElementPanel {
    pub fn ApplyStyles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearCachedElementUnderPointer(
        &mut self,
        pointerId: i32,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCachedElementUnderPointer", (pointerId, triggerEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn CommitElementUnderPointers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CommitElementUnderPointers", ())?;
        Ok(__cordl_ret.into())
    }
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
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTopElementUnderPointer(
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
    pub fn GetUpdater(
        &mut self,
        phase: crate::UnityEngine::UIElements::VisualTreeUpdatePhase,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IVisualTreeUpdater>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualTreeUpdater,
        > = __cordl_object.invoke("GetUpdater", (phase))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeAtlasChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeAtlasChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeBeforeUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeHierarchyChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        changeType: crate::UnityEngine::UIElements::HierarchyChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeHierarchyChanged", (ve, changeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeUpdateMaterial(
        &mut self,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeUpdateMaterial", (mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnVersionChanged(
        &mut self,
        ele: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        changeTypeFlag: crate::UnityEngine::UIElements::VersionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnVersionChanged", (ele, changeTypeFlag))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pick(
        &mut self,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("Pick", (point))?;
        Ok(__cordl_ret.into())
    }
    pub fn PickAll(
        &mut self,
        point: crate::UnityEngine::Vector2,
        picked: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("PickAll", (point, picked))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecomputeTopElementUnderPointer(
        &mut self,
        pointerId: i32,
        pointerPos: crate::UnityEngine::Vector2,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
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
                "RecomputeTopElementUnderPointer",
                (pointerId, pointerPos, triggerEvent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Repaint(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Repaint", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEvent(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendEvent", (e, dispatchMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IGroupBox_OnOptionAdded(
        &mut self,
        option: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IGroupBox.OnOptionAdded", (option))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IGroupBox_OnOptionRemoved(
        &mut self,
        option: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IGroupBox.OnOptionRemoved", (option))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAnimations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateElementUnderPointers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateElementUnderPointers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFocus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateLayout", ())?;
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
    pub fn add_atlasChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_atlasChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_hierarchyChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::HierarchyEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hierarchyChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_panelDisposed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::BaseVisualElementPanel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_panelDisposed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_standardShaderChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_standardShaderChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_standardWorldSpaceShaderChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_standardWorldSpaceShaderChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IMGUIContainersCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_IMGUIContainersCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IMGUIEventInterests(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventInterests> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventInterests = __cordl_object
            .invoke("get_IMGUIEventInterests", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::AtlasBase,
        > = __cordl_object.invoke("get_atlas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clearSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::PanelClearSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::PanelClearSettings = __cordl_object
            .invoke("get_clearSettings", ())?;
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
    pub fn get_contextualMenuManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ContextualMenuManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ContextualMenuManager,
        > = __cordl_object.invoke("get_contextualMenuManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursorManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ICursorManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ICursorManager,
        > = __cordl_object.invoke("get_cursorManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dispatcher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventDispatcher>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventDispatcher,
        > = __cordl_object.invoke("get_dispatcher", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duringLayoutPhase(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_duringLayoutPhase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusController,
        > = __cordl_object.invoke("get_focusController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_getViewDataDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::GetViewDataDictionary>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GetViewDataDictionary,
        > = __cordl_object.invoke("get_getViewDataDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hierarchyVersion(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_hierarchyVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ownerObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ScriptableObject,
        > = __cordl_object.invoke("get_ownerObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_referenceSpritePixelsPerUnit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_referenceSpritePixelsPerUnit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_repaintData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::RepaintData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::RepaintData,
        > = __cordl_object.invoke("get_repaintData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rootIMGUIContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IMGUIContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IMGUIContainer,
        > = __cordl_object.invoke("get_rootIMGUIContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_saveViewData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::SavePersistentViewData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SavePersistentViewData,
        > = __cordl_object.invoke("get_saveViewData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scaledPixelsPerPoint(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scaledPixelsPerPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IScheduler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IScheduler,
        > = __cordl_object.invoke("get_scheduler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_standardShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = __cordl_object
            .invoke("get_standardShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_standardWorldSpaceShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = __cordl_object
            .invoke("get_standardWorldSpaceShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleAnimationSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStylePropertyAnimationSystem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStylePropertyAnimationSystem,
        > = __cordl_object.invoke("get_styleAnimationSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uiElementsBridge(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIElementsBridge>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIElementsBridge,
        > = __cordl_object.invoke("get_uiElementsBridge", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_visualTree(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_visualTree", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_atlasChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_atlasChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_hierarchyChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::HierarchyEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hierarchyChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_panelDisposed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::BaseVisualElementPanel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_panelDisposed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_standardShaderChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_standardShaderChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_standardWorldSpaceShaderChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_standardWorldSpaceShaderChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IMGUIContainersCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IMGUIContainersCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IMGUIEventInterests(
        &mut self,
        value: crate::UnityEngine::EventInterests,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IMGUIEventInterests", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_atlas(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlas", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clearSettings(
        &mut self,
        value: crate::UnityEngine::UIElements::PanelClearSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clearSettings", (value))?;
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
    pub fn set_contextualMenuManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ContextualMenuManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contextualMenuManager", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cursorManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ICursorManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cursorManager", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_dispatcher(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventDispatcher>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dispatcher", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_duringLayoutPhase(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_duringLayoutPhase", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_focusController(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_focusController", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ownerObject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ownerObject", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pixelsPerPoint(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pixelsPerPoint", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_referenceSpritePixelsPerUnit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_referenceSpritePixelsPerUnit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_repaintData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::RepaintData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_repaintData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_styleAnimationSystem(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStylePropertyAnimationSystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_styleAnimationSystem", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl AsRef<crate::UnityEngine::UIElements::IGroupBox>
for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IGroupBox {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl AsMut<crate::UnityEngine::UIElements::IGroupBox>
for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IGroupBox {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl AsRef<crate::UnityEngine::UIElements::IPanel>
for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IPanel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualElementPanel")]
impl AsMut<crate::UnityEngine::UIElements::IPanel>
for crate::UnityEngine::UIElements::BaseVisualElementPanel {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IPanel {
        unsafe { std::mem::transmute(self) }
    }
}
