#[cfg(feature = "UnityEngine+UIElements+VisualElementAnimationSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementAnimationSystem {
    __cordl_parent: crate::UnityEngine::UIElements::BaseVisualTreeUpdater,
    pub m_Animations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
        >,
    >,
    pub m_IterationList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
        >,
    >,
    pub m_HasNewAnimations: bool,
    pub m_IterationListDirty: bool,
    pub lastUpdate: i64,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementAnimationSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElementAnimationSystem => "UnityEngine.UIElements"
    ."VisualElementAnimationSystem"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementAnimationSystem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementAnimationSystem {
    type Target = crate::UnityEngine::UIElements::BaseVisualTreeUpdater;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementAnimationSystem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElementAnimationSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementAnimationSystem")]
impl crate::UnityEngine::UIElements::VisualElementAnimationSystem {
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
    pub fn RegisterAnimation(
        &mut self,
        anim: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterAnimation", (anim))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterAnimations(
        &mut self,
        anims: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterAnimations", (anims))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterAnimation(
        &mut self,
        anim: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterAnimation", (anim))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterAnimations(
        &mut self,
        anims: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterAnimations", (anims))?;
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
    pub fn get_stylePropertyAnimationProfilerMarker() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerMarker,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarker = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_stylePropertyAnimationProfilerMarker", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementAnimationSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementAnimationSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
