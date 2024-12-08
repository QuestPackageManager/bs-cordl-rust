#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
#[repr(C)]
#[derive(Debug)]
pub struct ElementSelection {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::ElementSelection =>
    "UnityEngine.ProBuilder.MeshOperations"."ElementSelection"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
impl crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    pub const k_MaxHoleIterations: i32 = 2048i32;
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection___c;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection+__c__DisplayClass25_0"
    )]
    pub type __c__DisplayClass25_0 = crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection___c__DisplayClass25_0;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection+__c__DisplayClass27_0"
    )]
    pub type __c__DisplayClass27_0 = crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection___c__DisplayClass27_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
