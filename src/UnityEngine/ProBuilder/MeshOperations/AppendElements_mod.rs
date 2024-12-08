#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
#[repr(C)]
#[derive(Debug)]
pub struct AppendElements {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::AppendElements =>
    "UnityEngine.ProBuilder.MeshOperations"."AppendElements"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements+__c__DisplayClass17_0"
    )]
    pub type __c__DisplayClass17_0 = crate::UnityEngine::ProBuilder::MeshOperations::AppendElements___c__DisplayClass17_0;
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::AppendElements___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
