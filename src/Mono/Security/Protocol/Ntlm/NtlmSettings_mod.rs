#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct NtlmSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Security.Protocol.Ntlm";
    const CLASS_NAME: &'static str = "NtlmSettings";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
impl crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    pub fn get_DefaultAuthLevel() -> quest_hook::libil2cpp::Result<
        crate::Mono::Security::Protocol::Ntlm::NtlmAuthLevel,
    > {
        let __cordl_ret: crate::Mono::Security::Protocol::Ntlm::NtlmAuthLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultAuthLevel", ())?;
        Ok(__cordl_ret.into())
    }
}
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
