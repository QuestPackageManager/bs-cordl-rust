#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtrudeElements {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements =>
    "UnityEngine.ProBuilder.MeshOperations"."ExtrudeElements"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}