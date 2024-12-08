#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1VerifierFactoryProvider {
    __cordl_parent: crate::System::Object,
    pub publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider =>
    "Org.BouncyCastle.Crypto.Operators"."Asn1VerifierFactoryProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    pub fn _ctor(
        &mut self,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn CreateVerifierFactory(
        &mut self,
        algorithmDetails: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IVerifierFactory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IVerifierFactory = __cordl_object
            .invoke("CreateVerifierFactory", (algorithmDetails))?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureAlgNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("get_SignatureAlgNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicKey))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
