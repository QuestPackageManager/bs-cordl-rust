#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931SecureRandomBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct X931SecureRandomBuilder {
    __cordl_parent: crate::System::Object,
    pub mRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub mEntropySourceProvider: *mut crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
    pub mDateTimeVector: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931SecureRandomBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::X931SecureRandomBuilder =>
    "Org.BouncyCastle.Crypto.Prng"."X931SecureRandomBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931SecureRandomBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::X931SecureRandomBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931SecureRandomBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::X931SecureRandomBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931SecureRandomBuilder")]
impl crate::Org::BouncyCastle::Crypto::Prng::X931SecureRandomBuilder {
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SecureRandom__cordl_bool1(
        &mut self,
        entropySource: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entropySource, predictionResistant))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEntropySourceProvider2(
        &mut self,
        entropySourceProvider: *mut crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entropySourceProvider))?;
        Ok(__cordl_ret)
    }
    pub fn SetDateTimeVector(
        &mut self,
        dateTimeVector: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::X931SecureRandomBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::X931SecureRandomBuilder = __cordl_object
            .invoke("SetDateTimeVector", (dateTimeVector))?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::X931SecureRandom,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::X931SecureRandom = __cordl_object
            .invoke("Build", (engine, key, predictionResistant))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SecureRandom__cordl_bool1(
        entropySource: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entropySource, predictionResistant))?;
        Ok(__cordl_object)
    }
    pub fn New_IEntropySourceProvider2(
        entropySourceProvider: *mut crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entropySourceProvider))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931SecureRandomBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::X931SecureRandomBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
