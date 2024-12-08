#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Client")]
#[repr(C)]
#[derive(Debug)]
pub struct Srp6Client {
    __cordl_parent: crate::System::Object,
    pub N: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub g: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub privA: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub pubA: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub B: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub x: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub u: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub S: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub M1: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub M2: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub Key: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Client")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Client =>
    "Org.BouncyCastle.Crypto.Agreement.Srp"."Srp6Client"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Client")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Client {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Client")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Client")]
impl crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Client {
    pub fn CalculateClientEvidenceMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateClientEvidenceMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateS(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateS", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateSecret(
        &mut self,
        serverB: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateSecret", (serverB))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateSessionKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateSessionKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateClientCredentials(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("GenerateClientCredentials", (salt, identity, password))?;
        Ok(__cordl_ret)
    }
    pub fn Init_BigInteger_BigInteger_IDigest_SecureRandom0(
        &mut self,
        N: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (N, g, digest, random))?;
        Ok(__cordl_ret)
    }
    pub fn Init_Srp6GroupParameters_IDigest_SecureRandom1(
        &mut self,
        group: *mut crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (group, digest, random))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SelectPrivateValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("SelectPrivateValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn VerifyServerEvidenceMessage(
        &mut self,
        serverM2: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifyServerEvidenceMessage", (serverM2))?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Client")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Client {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
