#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931Rng")]
#[repr(C)]
#[derive(Debug)]
pub struct X931Rng {
    __cordl_parent: crate::System::Object,
    pub mEngine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    pub mEntropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    pub mDT: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mI: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mR: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mReseedCounter: i64,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931Rng")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Prng::X931Rng =>
    "Org.BouncyCastle.Crypto.Prng"."X931Rng"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931Rng")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::X931Rng {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931Rng")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::X931Rng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931Rng")]
impl crate::Org::BouncyCastle::Crypto::Prng::X931Rng {
    pub const BLOCK128_MAX_BITS_REQUEST: i32 = 262144i32;
    pub const BLOCK128_RESEED_MAX: i64 = 8388608i64;
    pub const BLOCK64_MAX_BITS_REQUEST: i32 = 4096i32;
    pub const BLOCK64_RESEED_MAX: i64 = 32768i64;
    pub fn Generate(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Generate", (output, predictionResistant))?;
        Ok(__cordl_ret)
    }
    pub fn get_EntropySource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IEntropySource = __cordl_object
            .invoke("get_EntropySource", ())?;
        Ok(__cordl_ret)
    }
    pub fn Increment(
        &mut self,
        val: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Increment", (val))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        dateTimeVector: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (engine, dateTimeVector, entropySource))?;
        Ok(__cordl_ret)
    }
    pub fn Reseed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reseed", ())?;
        Ok(__cordl_ret)
    }
    pub fn Process(
        &mut self,
        res: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        a: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Process", (res, a, b))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        dateTimeVector: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (engine, dateTimeVector, entropySource))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+X931Rng")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::X931Rng {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
