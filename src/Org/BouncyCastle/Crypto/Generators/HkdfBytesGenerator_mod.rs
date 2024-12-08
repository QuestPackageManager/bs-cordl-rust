#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+HkdfBytesGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct HkdfBytesGenerator {
    __cordl_parent: crate::System::Object,
    pub hMacHash: *mut crate::Org::BouncyCastle::Crypto::Macs::HMac,
    pub hashLen: i32,
    pub info: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub currentT: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub generatedBytes: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+HkdfBytesGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::HkdfBytesGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."HkdfBytesGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+HkdfBytesGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::HkdfBytesGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+HkdfBytesGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::HkdfBytesGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+HkdfBytesGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::HkdfBytesGenerator {
    pub fn ExpandNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn Extract(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ikm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter = __cordl_object
            .invoke("Extract", (salt, ikm))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateBytes(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GenerateBytes", (output, outOff, len))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Crypto::IDerivationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hash))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hash))?;
        Ok(__cordl_ret)
    }
    pub fn get_Digest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::IDigest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IDigest = __cordl_object
            .invoke("get_Digest", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+HkdfBytesGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::HkdfBytesGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}