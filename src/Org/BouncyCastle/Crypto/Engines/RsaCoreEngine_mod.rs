#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+RsaCoreEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaCoreEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub key: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
    >,
    pub forEncryption: bool,
    pub bitSize: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+RsaCoreEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Engines::RsaCoreEngine =>
    "Org.BouncyCastle.Crypto.Engines"."RsaCoreEngine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+RsaCoreEngine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::RsaCoreEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+RsaCoreEngine")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Engines::RsaCoreEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+RsaCoreEngine")]
impl crate::Org::BouncyCastle::Crypto::Engines::RsaCoreEngine {
    pub fn CheckInitialised(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckInitialised", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertInput(
        &mut self,
        inBuf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("ConvertInput", (inBuf, inOff, inLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertOutput(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ConvertOutput", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInputBlockSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetOutputBlockSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        forEncryption: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forEncryption, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessBlock(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("ProcessBlock", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+RsaCoreEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::RsaCoreEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+RsaCoreEngine")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IRsa>
for crate::Org::BouncyCastle::Crypto::Engines::RsaCoreEngine {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IRsa {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+RsaCoreEngine")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IRsa>
for crate::Org::BouncyCastle::Crypto::Engines::RsaCoreEngine {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IRsa {
        unsafe { std::mem::transmute(self) }
    }
}
