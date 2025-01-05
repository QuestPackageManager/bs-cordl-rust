#[cfg(feature = "Mono+Security+Protocol+Ntlm+ChallengeResponse2")]
#[repr(C)]
#[derive(Debug)]
pub struct ChallengeResponse2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+ChallengeResponse2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Security::Protocol::Ntlm::ChallengeResponse2 => "Mono.Security.Protocol.Ntlm"
    ."ChallengeResponse2"
);
#[cfg(feature = "Mono+Security+Protocol+Ntlm+ChallengeResponse2")]
impl std::ops::Deref for crate::Mono::Security::Protocol::Ntlm::ChallengeResponse2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+ChallengeResponse2")]
impl std::ops::DerefMut for crate::Mono::Security::Protocol::Ntlm::ChallengeResponse2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+ChallengeResponse2")]
impl crate::Mono::Security::Protocol::Ntlm::ChallengeResponse2 {
    pub fn Compute(
        type2: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Protocol::Ntlm::Type2Message,
        >,
        level: crate::Mono::Security::Protocol::Ntlm::NtlmAuthLevel,
        username: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        domain: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lm: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        ntlm: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compute", (type2, level, username, password, domain, lm, ntlm))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compute_LM(
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compute_LM", (password, challenge))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compute_NTLM(
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compute_NTLM", (password, challenge))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compute_NTLM_Password(
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compute_NTLM_Password", (password))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compute_NTLMv2(
        type2: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Protocol::Ntlm::Type2Message,
        >,
        username: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        domain: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compute_NTLMv2", (type2, username, password, domain))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compute_NTLMv2_Session(
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        lm: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        ntlm: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compute_NTLMv2_Session", (password, challenge, lm, ntlm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResponse(
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pwd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResponse", (challenge, pwd))?;
        Ok(__cordl_ret.into())
    }
    pub fn PasswordToKey(
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PasswordToKey", (password, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareDESKey(
        key56bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareDESKey", (key56bits, position))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+ChallengeResponse2")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Protocol::Ntlm::ChallengeResponse2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
