#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
#[repr(C)]
#[derive(Debug)]
pub struct Bevel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshOperations::Bevel
    => "UnityEngine.ProBuilder.MeshOperations"."Bevel"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::Bevel___c;
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::UnityEngine::ProBuilder::MeshOperations::Bevel___c__DisplayClass0_0;
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel+__c__DisplayClass0_1")]
    pub type __c__DisplayClass0_1 = crate::UnityEngine::ProBuilder::MeshOperations::Bevel___c__DisplayClass0_1;
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel+__c__DisplayClass0_2")]
    pub type __c__DisplayClass0_2 = crate::UnityEngine::ProBuilder::MeshOperations::Bevel___c__DisplayClass0_2;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
