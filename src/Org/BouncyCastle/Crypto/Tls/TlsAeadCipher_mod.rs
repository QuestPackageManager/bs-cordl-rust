#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsAeadCipher {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub context: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    >,
    pub macSize: i32,
    pub record_iv_length: i32,
    pub encryptCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
    >,
    pub decryptCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
    >,
    pub encryptImplicitNonce: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub decryptImplicitNonce: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub nonceMode: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher =>
    "Org.BouncyCastle.Crypto.Tls"."TlsAeadCipher"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    pub const NONCE_DRAFT_CHACHA20_POLY1305: i32 = 2i32;
    pub const NONCE_RFC5288: i32 = 1i32;
    pub fn DecodeCiphertext(
        &mut self,
        seqNo: i64,
        _cordl_type: u8,
        ciphertext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object
            .invoke("DecodeCiphertext", (seqNo, _cordl_type, ciphertext, offset, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodePlaintext(
        &mut self,
        seqNo: i64,
        _cordl_type: u8,
        plaintext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object
            .invoke("EncodePlaintext", (seqNo, _cordl_type, plaintext, offset, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAdditionalData(
        &mut self,
        seqNo: i64,
        _cordl_type: u8,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetAdditionalData", (seqNo, _cordl_type, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlaintextLimit(
        &mut self,
        ciphertextLimit: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPlaintextLimit", (ciphertextLimit))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Gc_Gc_Gc_i32_i32_0(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        cipherKeySize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (context, clientWriteCipher, serverWriteCipher, cipherKeySize, macSize),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        cipherKeySize: i32,
        macSize: i32,
        nonceMode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    context,
                    clientWriteCipher,
                    serverWriteCipher,
                    cipherKeySize,
                    macSize,
                    nonceMode,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Gc_Gc_Gc_i32_i32_0(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        cipherKeySize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (context, clientWriteCipher, serverWriteCipher, cipherKeySize, macSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        cipherKeySize: i32,
        macSize: i32,
        nonceMode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    context,
                    clientWriteCipher,
                    serverWriteCipher,
                    cipherKeySize,
                    macSize,
                    nonceMode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>>
for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>>
for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
