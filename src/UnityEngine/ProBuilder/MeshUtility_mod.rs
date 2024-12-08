#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshUtility =>
    "UnityEngine.ProBuilder"."MeshUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
impl crate::UnityEngine::ProBuilder::MeshUtility {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshUtility+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshUtility___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::MeshUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
