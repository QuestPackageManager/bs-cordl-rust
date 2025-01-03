#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct IesEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub agree: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IBasicAgreement,
    >,
    pub kdf: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IDerivationFunction,
    >,
    pub mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    pub cipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::BufferedBlockCipher,
    >,
    pub macBuf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub forEncryption: bool,
    pub privParam: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::ICipherParameters,
    >,
    pub pubParam: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::ICipherParameters,
    >,
    pub param: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::IesParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Engines::IesEngine =>
    "Org.BouncyCastle.Crypto.Engines"."IesEngine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::IesEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Engines::IesEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
impl crate::Org::BouncyCastle::Crypto::Engines::IesEngine {
    pub fn DecryptBlock(
        &mut self,
        in_enc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        inLen: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("DecryptBlock", (in_enc, inOff, inLen, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptBlock(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        inLen: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("EncryptBlock", (input, inOff, inLen, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateKdfBytes(
        &mut self,
        kParam: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KdfParameters,
        >,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateKdfBytes", (kParam, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        forEncryption: bool,
        privParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
        pubParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
        iesParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (forEncryption, privParameters, pubParameters, iesParameters),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_BufferedBlockCipher1(
        agree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        >,
        kdf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        >,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::BufferedBlockCipher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (agree, kdf, mac, cipher))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IBasicAgreement_IDerivationFunction_IMac0(
        agree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        >,
        kdf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        >,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (agree, kdf, mac))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessBlock(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ProcessBlock", (input, inOff, inLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BufferedBlockCipher1(
        &mut self,
        agree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        >,
        kdf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        >,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::BufferedBlockCipher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (agree, kdf, mac, cipher))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IBasicAgreement_IDerivationFunction_IMac0(
        &mut self,
        agree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        >,
        kdf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        >,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (agree, kdf, mac))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::IesEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
