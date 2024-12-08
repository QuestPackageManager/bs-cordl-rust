#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshHandles {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshHandles =>
    "UnityEngine.ProBuilder"."MeshHandles"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshHandles {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshHandles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl crate::UnityEngine::ProBuilder::MeshHandles {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshHandles+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshHandles___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::MeshHandles {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
