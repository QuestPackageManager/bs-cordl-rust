#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct NtlmSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Protocol::Ntlm::NtlmSettings =>
    "Mono.Security.Protocol.Ntlm"."NtlmSettings"
);
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
impl std::ops::Deref for crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
impl std::ops::DerefMut for crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
impl crate::Mono::Security::Protocol::Ntlm::NtlmSettings {}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
