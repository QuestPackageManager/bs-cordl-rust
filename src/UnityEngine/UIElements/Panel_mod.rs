#[cfg(feature = "UnityEngine+UIElements+Panel")]
#[repr(C)]
#[derive(Debug)]
pub struct Panel {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseVisualElementPanel,
    >,
    pub m_RootContainer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_VisualTreeUpdater: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualTreeUpdater,
    >,
    pub m_StylePropertyAnimationSystem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IStylePropertyAnimationSystem,
    >,
    pub m_PanelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Version: u32,
    pub m_RepaintVersion: u32,
    pub m_HierarchyVersion: u32,
    pub m_MarkerBeforeUpdate: crate::Unity::Profiling::ProfilerMarker,
    pub m_MarkerUpdate: crate::Unity::Profiling::ProfilerMarker,
    pub m_MarkerLayout: crate::Unity::Profiling::ProfilerMarker,
    pub m_MarkerBindings: crate::Unity::Profiling::ProfilerMarker,
    pub m_MarkerAnimations: crate::Unity::Profiling::ProfilerMarker,
    pub _dispatcher_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventDispatcher,
    >,
    pub m_Scheduler: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TimerEventScheduler,
    >,
    pub _ownerObject_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ScriptableObject,
    >,
    pub _contextType_k__BackingField: crate::UnityEngine::UIElements::ContextType,
    pub _saveViewData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::SavePersistentViewData,
    >,
    pub _getViewDataDictionary_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::GetViewDataDictionary,
    >,
    pub _focusController_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::FocusController,
    >,
    pub _IMGUIEventInterests_k__BackingField: crate::UnityEngine::EventInterests,
    pub m_JustReceivedFocus: bool,
    pub _IMGUIContainersCount_k__BackingField: i32,
    pub _rootIMGUIContainer_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IMGUIContainer,
    >,
    pub m_StandardShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub m_Atlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
    pub m_ValidatingLayout: bool,
}
#[cfg(feature = "UnityEngine+UIElements+Panel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Panel =>
    "UnityEngine.UIElements"."Panel"
);
#[cfg(feature = "UnityEngine+UIElements+Panel")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Panel {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseVisualElementPanel,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Panel")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Panel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Panel")]
impl crate::UnityEngine::UIElements::Panel {
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
    pub fn Blur(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blur", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMarkers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateMarkers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultTimeSinceStartupMs() -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultTimeSinceStartupMs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
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
    pub fn Focus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Focus", ())?;
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
    pub fn LoadResource(
        pathName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dpiScaling: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadResource", (pathName, _cordl_type, dpiScaling))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        ownerObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        contextType: crate::UnityEngine::UIElements::ContextType,
        dispatcher: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventDispatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ownerObject, contextType, dispatcher))?;
        Ok(__cordl_object.into())
    }
    pub fn OnVersionChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        versionChangeType: crate::UnityEngine::UIElements::VersionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnVersionChanged", (ve, versionChangeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformPick(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        point: crate::UnityEngine::Vector2,
        picked: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        >,
        includeIgnoredElement: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PerformPick", (root, point, picked, includeIgnoredElement))?;
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
    pub fn PickAll_Gc_Vector2_Gc__cordl_bool0(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        point: crate::UnityEngine::Vector2,
        picked: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        >,
        includeIgnoredElement: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PickAll", (root, point, picked, includeIgnoredElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn PickAll_Vector2_Gc1(
        &mut self,
        point: crate::UnityEngine::Vector2,
        picked: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
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
    pub fn TimeSinceStartupMs() -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TimeSinceStartupMs", ())?;
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
    pub fn UpdateForRepaint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateForRepaint", ())?;
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
    pub fn _Pick_g__PixelOf_101_0(
        p: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        let __cordl_ret: crate::UnityEngine::Vector2Int = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("<Pick>g__PixelOf|101_0", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ownerObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        contextType: crate::UnityEngine::UIElements::ContextType,
        dispatcher: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventDispatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ownerObject, contextType, dispatcher))?;
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
    pub fn get_TimeSinceStartup() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimeMsFunction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TimeMsFunction,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_TimeSinceStartup", ())?;
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
    pub fn get_loadResourceFunc() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::LoadResourceFunction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::LoadResourceFunction,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_loadResourceFunc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
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
    pub fn get_timerEventScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimerEventScheduler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TimerEventScheduler,
        > = __cordl_object.invoke("get_timerEventScheduler", ())?;
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
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
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
#[cfg(feature = "UnityEngine+UIElements+Panel")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Panel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
