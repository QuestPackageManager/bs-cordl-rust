#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct CallbackHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::Private::CallbackHelpers =>
    "Mono.Net.Security.Private"."CallbackHelpers"
);
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
impl std::ops::Deref for crate::Mono::Net::Security::Private::CallbackHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
impl std::ops::DerefMut for crate::Mono::Net::Security::Private::CallbackHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
impl crate::Mono::Net::Security::Private::CallbackHelpers {
    #[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::Mono::Net::Security::Private::CallbackHelpers___c__DisplayClass0_0;
    #[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::Mono::Net::Security::Private::CallbackHelpers___c__DisplayClass6_0;
}
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::Private::CallbackHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
