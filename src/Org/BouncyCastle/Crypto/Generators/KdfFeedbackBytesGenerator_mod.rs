#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfFeedbackBytesGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct KdfFeedbackBytesGenerator {
    __cordl_parent: crate::System::Object,
    pub prf: *mut crate::Org::BouncyCastle::Crypto::IMac,
    pub h: i32,
    pub fixedInputData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub maxSizeExcl: i32,
    pub ios: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub useCounter: bool,
    pub generatedBytes: i32,
    pub k: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfFeedbackBytesGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::KdfFeedbackBytesGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."KdfFeedbackBytesGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfFeedbackBytesGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::KdfFeedbackBytesGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfFeedbackBytesGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::KdfFeedbackBytesGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfFeedbackBytesGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::KdfFeedbackBytesGenerator {
    pub fn generateNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("generateNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::IMac> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IMac = __cordl_object
            .invoke("GetMac", ())?;
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
    pub fn GenerateBytes(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GenerateBytes", (output, outOff, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        prf: *mut crate::Org::BouncyCastle::Crypto::IMac,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prf))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        prf: *mut crate::Org::BouncyCastle::Crypto::IMac,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prf))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfFeedbackBytesGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::KdfFeedbackBytesGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
