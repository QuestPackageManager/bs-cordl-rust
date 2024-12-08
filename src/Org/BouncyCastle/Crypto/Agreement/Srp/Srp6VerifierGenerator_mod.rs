#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6VerifierGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct Srp6VerifierGenerator {
    __cordl_parent: crate::System::Object,
    pub N: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub g: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6VerifierGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator =>
    "Org.BouncyCastle.Crypto.Agreement.Srp"."Srp6VerifierGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6VerifierGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6VerifierGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6VerifierGenerator")]
impl crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator {
    pub fn GenerateVerifier(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("GenerateVerifier", (salt, identity, password))?;
        Ok(__cordl_ret)
    }
    pub fn Init_BigInteger_BigInteger_IDigest0(
        &mut self,
        N: *mut crate::Org::BouncyCastle::Math::BigInteger,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (N, g, digest))?;
        Ok(__cordl_ret)
    }
    pub fn Init_Srp6GroupParameters_IDigest1(
        &mut self,
        group: *mut crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (group, digest))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6VerifierGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
