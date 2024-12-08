#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1CipherBuilderWithKey {
    __cordl_parent: crate::System::Object,
    pub encKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    pub algorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey =>
    "Org.BouncyCastle.Crypto.Operators"."Asn1CipherBuilderWithKey"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey {
    pub fn _ctor(
        &mut self,
        encryptionOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        keySize: i32,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encryptionOID, keySize, random))?;
        Ok(__cordl_ret)
    }
    pub fn GetMaxOutputSize(
        &mut self,
        inputLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMaxOutputSize", (inputLen))?;
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
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters = __cordl_object
            .invoke("get_Key", ())?;
        Ok(__cordl_ret)
    }
    pub fn BuildCipher(
        &mut self,
        stream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::ICipher> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipher = __cordl_object
            .invoke("BuildCipher", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        encryptionOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        keySize: i32,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptionOID, keySize, random))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
