#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsCipherFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTlsCipherFactory {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsCipherFactory,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsCipherFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DefaultTlsCipherFactory =>
    "Org.BouncyCastle.Crypto.Tls"."DefaultTlsCipherFactory"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsCipherFactory")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsCipherFactory {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsCipherFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsCipherFactory")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsCipherFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsCipherFactory")]
impl crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsCipherFactory {
    pub fn CreateAESCipher(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        cipherKeySize: i32,
        macAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher = __cordl_object
            .invoke("CreateAESCipher", (context, cipherKeySize, macAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAeadBlockCipher_Aes_Ccm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher = __cordl_object
            .invoke("CreateAeadBlockCipher_Aes_Ccm", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateAeadBlockCipher_Aes_Gcm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher = __cordl_object
            .invoke("CreateAeadBlockCipher_Aes_Gcm", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateAeadBlockCipher_Aes_Ocb(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher = __cordl_object
            .invoke("CreateAeadBlockCipher_Aes_Ocb", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateAeadBlockCipher_Camellia_Gcm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher = __cordl_object
            .invoke("CreateAeadBlockCipher_Camellia_Gcm", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateAesBlockCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher = __cordl_object
            .invoke("CreateAesBlockCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateAesEngine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher = __cordl_object
            .invoke("CreateAesEngine", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateCamelliaBlockCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher = __cordl_object
            .invoke("CreateCamelliaBlockCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateCamelliaCipher(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        cipherKeySize: i32,
        macAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher = __cordl_object
            .invoke("CreateCamelliaCipher", (context, cipherKeySize, macAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCamelliaEngine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher = __cordl_object
            .invoke("CreateCamelliaEngine", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateChaCha20Poly1305(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher = __cordl_object
            .invoke("CreateChaCha20Poly1305", (context))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCipher(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        encryptionAlgorithm: i32,
        macAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher = __cordl_object
            .invoke("CreateCipher", (context, encryptionAlgorithm, macAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCipher_Aes_Ccm(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        cipherKeySize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher = __cordl_object
            .invoke("CreateCipher_Aes_Ccm", (context, cipherKeySize, macSize))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCipher_Aes_Gcm(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        cipherKeySize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher = __cordl_object
            .invoke("CreateCipher_Aes_Gcm", (context, cipherKeySize, macSize))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCipher_Aes_Ocb(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        cipherKeySize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher = __cordl_object
            .invoke("CreateCipher_Aes_Ocb", (context, cipherKeySize, macSize))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCipher_Camellia_Gcm(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        cipherKeySize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher = __cordl_object
            .invoke("CreateCipher_Camellia_Gcm", (context, cipherKeySize, macSize))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDesEdeBlockCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher = __cordl_object
            .invoke("CreateDesEdeBlockCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateDesEdeCipher(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        macAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher = __cordl_object
            .invoke("CreateDesEdeCipher", (context, macAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn CreateHMacDigest(
        &mut self,
        macAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::IDigest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IDigest = __cordl_object
            .invoke("CreateHMacDigest", (macAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn CreateNullCipher(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        macAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsNullCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsNullCipher = __cordl_object
            .invoke("CreateNullCipher", (context, macAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn CreateRC4Cipher(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        cipherKeySize: i32,
        macAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsStreamCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsStreamCipher = __cordl_object
            .invoke("CreateRC4Cipher", (context, cipherKeySize, macAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn CreateRC4StreamCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IStreamCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IStreamCipher = __cordl_object
            .invoke("CreateRC4StreamCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateSeedBlockCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher = __cordl_object
            .invoke("CreateSeedBlockCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateSeedCipher(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        macAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher = __cordl_object
            .invoke("CreateSeedCipher", (context, macAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsCipherFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsCipherFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
