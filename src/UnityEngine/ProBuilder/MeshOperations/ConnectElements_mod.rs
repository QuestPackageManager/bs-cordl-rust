#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectElements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::ConnectElements =>
    "UnityEngine.ProBuilder.MeshOperations"."ConnectElements"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements___c;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements+__c__DisplayClass2_0"
    )]
    pub type __c__DisplayClass2_0 = crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements___c__DisplayClass2_0;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements+__c__DisplayClass3_0"
    )]
    pub type __c__DisplayClass3_0 = crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements___c__DisplayClass3_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
