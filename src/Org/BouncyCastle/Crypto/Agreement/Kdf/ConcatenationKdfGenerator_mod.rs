#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+ConcatenationKdfGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcatenationKdfGenerator {
    __cordl_parent: crate::System::Object,
    pub mDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub mShared: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mOtherInfo: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mHLen: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+ConcatenationKdfGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::Kdf::ConcatenationKdfGenerator =>
    "Org.BouncyCastle.Crypto.Agreement.Kdf"."ConcatenationKdfGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+ConcatenationKdfGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::Kdf::ConcatenationKdfGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+ConcatenationKdfGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::Kdf::ConcatenationKdfGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+ConcatenationKdfGenerator")]
impl crate::Org::BouncyCastle::Crypto::Agreement::Kdf::ConcatenationKdfGenerator {
    pub fn GenerateBytes(
        &mut self,
        outBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GenerateBytes", (outBytes, outOff, len))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        param: *mut crate::Org::BouncyCastle::Crypto::IDerivationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (param))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Kdf+ConcatenationKdfGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::Kdf::ConcatenationKdfGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
