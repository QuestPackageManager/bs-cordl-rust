#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MergeElements")]
#[repr(C)]
#[derive(Debug)]
pub struct MergeElements {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MergeElements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::MergeElements =>
    "UnityEngine.ProBuilder.MeshOperations"."MergeElements"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MergeElements")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::MergeElements {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MergeElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::MergeElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MergeElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::MergeElements {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MergeElements+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::MergeElements___c;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+MergeElements+__c__DisplayClass0_0"
    )]
    pub type __c__DisplayClass0_0 = crate::UnityEngine::ProBuilder::MeshOperations::MergeElements___c__DisplayClass0_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MergeElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::MergeElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
