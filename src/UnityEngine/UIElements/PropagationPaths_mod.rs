#[cfg(feature = "UnityEngine+UIElements+PropagationPaths")]
#[repr(C)]
#[derive(Debug)]
pub struct PropagationPaths {
    __cordl_parent: crate::System::Object,
    pub trickleDownPath: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub targetElements: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub bubbleUpPath: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+PropagationPaths")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PropagationPaths =>
    "UnityEngine.UIElements"."PropagationPaths"
);
#[cfg(feature = "UnityEngine+UIElements+PropagationPaths")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PropagationPaths {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PropagationPaths")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PropagationPaths {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PropagationPaths")]
impl crate::UnityEngine::UIElements::PropagationPaths {
    pub const k_DefaultPropagationDepth: i32 = 16i32;
    pub const k_DefaultTargetCount: i32 = 4i32;
    #[cfg(feature = "UnityEngine+UIElements+PropagationPaths+__c")]
    pub type __c = crate::UnityEngine::UIElements::PropagationPaths___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
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
}
#[cfg(feature = "UnityEngine+UIElements+PropagationPaths")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PropagationPaths {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
