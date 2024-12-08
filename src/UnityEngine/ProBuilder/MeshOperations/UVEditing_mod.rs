#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing")]
#[repr(C)]
#[derive(Debug)]
pub struct UVEditing {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::UVEditing =>
    "UnityEngine.ProBuilder.MeshOperations"."UVEditing"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::UVEditing {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::UVEditing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing")]
impl crate::UnityEngine::ProBuilder::MeshOperations::UVEditing {
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing+__c__DisplayClass4_0"
    )]
    pub type __c__DisplayClass4_0 = crate::UnityEngine::ProBuilder::MeshOperations::UVEditing___c__DisplayClass4_0;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing+__c__DisplayClass0_0"
    )]
    pub type __c__DisplayClass0_0 = crate::UnityEngine::ProBuilder::MeshOperations::UVEditing___c__DisplayClass0_0;
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::UVEditing___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+UVEditing")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::UVEditing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
