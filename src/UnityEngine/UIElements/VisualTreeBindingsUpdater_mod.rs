#[cfg(feature = "UnityEngine+UIElements+VisualTreeBindingsUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeBindingsUpdater {
    __cordl_parent: crate::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater,
    pub m_ElementsWithBindings: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_ElementsToAdd: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_ElementsToRemove: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_LastUpdateTime: i64,
    pub m_ElementsToBind: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _temporaryObjectCache_k__BackingField: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub updatedBindings: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::IBinding,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeBindingsUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeBindingsUpdater => "UnityEngine.UIElements"
    ."VisualTreeBindingsUpdater"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeBindingsUpdater")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualTreeBindingsUpdater {
    type Target = crate::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeBindingsUpdater")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualTreeBindingsUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeBindingsUpdater")]
impl crate::UnityEngine::UIElements::VisualTreeBindingsUpdater {
    pub fn GetBindingObjectFromElement(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::IBinding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IBinding = __cordl_object
            .invoke("GetBindingObjectFromElement", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnHierarchyChange(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        _cordl_type: crate::UnityEngine::UIElements::HierarchyChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHierarchyChange", (ve, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn OnVersionChanged(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        versionChangeType: crate::UnityEngine::UIElements::VersionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnVersionChanged", (ve, versionChangeType))?;
        Ok(__cordl_ret)
    }
    pub fn PerformTrackingOperations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformTrackingOperations", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartTracking(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartTracking", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn StartTrackingRecursive(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartTrackingRecursive", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn StopTracking(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopTracking", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn StopTrackingRecursive(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopTrackingRecursive", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_profilerMarker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerMarker> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarker = __cordl_object
            .invoke("get_profilerMarker", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_temporaryObjectCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_temporaryObjectCache", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeBindingsUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeBindingsUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
