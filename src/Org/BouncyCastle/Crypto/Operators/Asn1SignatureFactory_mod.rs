#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1SignatureFactory {
    __cordl_parent: crate::System::Object,
    pub algID: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub algorithm: *mut crate::System::String,
    pub privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory =>
    "Org.BouncyCastle.Crypto.Operators"."Asn1SignatureFactory"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory {
    pub fn CreateCalculator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IStreamCalculator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IStreamCalculator = __cordl_object
            .invoke("CreateCalculator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_SecureRandom1(
        algorithm: *mut crate::System::String,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, privateKey, random))?;
        Ok(__cordl_object)
    }
    pub fn New_String_AsymmetricKeyParameter0(
        algorithm: *mut crate::System::String,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, privateKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_SecureRandom1(
        &mut self,
        algorithm: *mut crate::System::String,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, privateKey, random))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_AsymmetricKeyParameter0(
        &mut self,
        algorithm: *mut crate::System::String,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_AlgorithmDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_AlgorithmDetails", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
