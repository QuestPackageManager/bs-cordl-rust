#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct KdfCounterBytesGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub prf: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    pub h: i32,
    pub fixedInputDataCtrPrefix: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub fixedInputData_afterCtr: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub maxSizeExcl: i32,
    pub ios: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub generatedBytes: i32,
    pub k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."KdfCounterBytesGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator {
    pub fn GenerateBytes(
        &mut self,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GenerateBytes", (output, outOff, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IMac,
        > = __cordl_object.invoke("GetMac", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        param: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDerivationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (param))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        prf: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prf))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        prf: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prf))?;
        Ok(__cordl_ret.into())
    }
    pub fn generateNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("generateNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Digest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = __cordl_object.invoke("get_Digest", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IDerivationFunction>
for crate::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IDerivationFunction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IDerivationFunction>
for crate::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IDerivationFunction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IMacDerivationFunction>
for crate::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IMacDerivationFunction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+KdfCounterBytesGenerator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IMacDerivationFunction>
for crate::Org::BouncyCastle::Crypto::Generators::KdfCounterBytesGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::IMacDerivationFunction {
        unsafe { std::mem::transmute(self) }
    }
}
