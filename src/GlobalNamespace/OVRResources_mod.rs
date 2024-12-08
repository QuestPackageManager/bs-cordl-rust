#[cfg(feature = "OVRResources")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRResources {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "OVRResources")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRResources => ""."OVRResources"
);
#[cfg(feature = "OVRResources")]
impl std::ops::Deref for OVRResources {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRResources")]
impl std::ops::DerefMut for OVRResources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRResources")]
impl OVRResources {
    #[cfg(feature = "OVRResources+__c__DisplayClass3_0_1")]
    pub type __c__DisplayClass3_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRResources___c__DisplayClass3_0_1<
        T,
    >;
    #[cfg(feature = "OVRResources+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::GlobalNamespace::OVRResources___c__DisplayClass2_0;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRResources")]
impl quest_hook::libil2cpp::ObjectType for OVRResources {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
