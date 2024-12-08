#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements")]
#[repr(C)]
#[derive(Debug)]
pub struct DeleteElements {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::DeleteElements =>
    "UnityEngine.ProBuilder.MeshOperations"."DeleteElements"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::DeleteElements {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::DeleteElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::DeleteElements {
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements+__c__DisplayClass2_0"
    )]
    pub type __c__DisplayClass2_0 = crate::UnityEngine::ProBuilder::MeshOperations::DeleteElements___c__DisplayClass2_0;
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::DeleteElements___c;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements+__c__DisplayClass0_0"
    )]
    pub type __c__DisplayClass0_0 = crate::UnityEngine::ProBuilder::MeshOperations::DeleteElements___c__DisplayClass0_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+DeleteElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::DeleteElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
