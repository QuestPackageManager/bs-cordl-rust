#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeStyleUpdater {
    __cordl_parent: crate::UnityEngine::UIElements::BaseVisualTreeUpdater,
    pub m_ApplyStyleUpdateList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
    >,
    pub m_TransitionPropertyUpdateList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
    >,
    pub m_IsApplyingStyles: bool,
    pub m_Version: u32,
    pub m_LastVersion: u32,
    pub m_StyleContextHierarchyTraversal: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualTreeStyleUpdaterTraversal,
    >,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualTreeStyleUpdater
    => "UnityEngine.UIElements"."VisualTreeStyleUpdater"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdater")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualTreeStyleUpdater {
    type Target = crate::UnityEngine::UIElements::BaseVisualTreeUpdater;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdater")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualTreeStyleUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdater")]
impl crate::UnityEngine::UIElements::VisualTreeStyleUpdater {
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_profilerMarker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerMarker> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarker = __cordl_object
            .invoke("get_profilerMarker", ())?;
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
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeStyleUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeStyleUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
