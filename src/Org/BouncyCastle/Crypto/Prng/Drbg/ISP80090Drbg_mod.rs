#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+ISP80090Drbg")]
#[repr(C)]
#[derive(Debug)]
pub struct ISP80090Drbg {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+ISP80090Drbg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg =>
    "Org.BouncyCastle.Crypto.Prng.Drbg"."ISP80090Drbg"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+ISP80090Drbg")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+ISP80090Drbg")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+ISP80090Drbg")]
impl crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg {
    pub fn Reseed(
        &mut self,
        additionalInput: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reseed", (additionalInput))?;
        Ok(__cordl_ret)
    }
    pub fn get_BlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Generate(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        additionalInput: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Generate", (output, additionalInput, predictionResistant))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+ISP80090Drbg")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
