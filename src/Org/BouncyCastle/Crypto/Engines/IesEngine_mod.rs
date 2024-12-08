#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct IesEngine {
    __cordl_parent: crate::System::Object,
    pub agree: *mut crate::Org::BouncyCastle::Crypto::IBasicAgreement,
    pub kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
    pub mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    pub cipher: *mut crate::Org::BouncyCastle::Crypto::BufferedBlockCipher,
    pub macBuf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub forEncryption: bool,
    pub privParam: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    pub pubParam: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    pub param: *mut crate::Org::BouncyCastle::Crypto::Parameters::IesParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Engines::IesEngine =>
    "Org.BouncyCastle.Crypto.Engines"."IesEngine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+IesEngine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::IesEngine {
    type Target = crate::System::Object;
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
        in_enc: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
        z: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DecryptBlock", (in_enc, inOff, inLen, z))?;
        Ok(__cordl_ret)
    }
    pub fn EncryptBlock(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
        z: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("EncryptBlock", (input, inOff, inLen, z))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateKdfBytes(
        &mut self,
        kParam: *mut crate::Org::BouncyCastle::Crypto::Parameters::KdfParameters,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateKdfBytes", (kParam, length))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        forEncryption: bool,
        privParameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        pubParameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        iesParameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (forEncryption, privParameters, pubParameters, iesParameters),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_BufferedBlockCipher1(
        agree: *mut crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
        cipher: *mut crate::Org::BouncyCastle::Crypto::BufferedBlockCipher,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (agree, kdf, mac, cipher))?;
        Ok(__cordl_object)
    }
    pub fn New_IBasicAgreement_IDerivationFunction_IMac0(
        agree: *mut crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (agree, kdf, mac))?;
        Ok(__cordl_object)
    }
    pub fn ProcessBlock(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ProcessBlock", (input, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BufferedBlockCipher1(
        &mut self,
        agree: *mut crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
        cipher: *mut crate::Org::BouncyCastle::Crypto::BufferedBlockCipher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (agree, kdf, mac, cipher))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IBasicAgreement_IDerivationFunction_IMac0(
        &mut self,
        agree: *mut crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (agree, kdf, mac))?;
        Ok(__cordl_ret)
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
