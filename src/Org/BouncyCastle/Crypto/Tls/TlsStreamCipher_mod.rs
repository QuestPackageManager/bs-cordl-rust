#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsStreamCipher")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsStreamCipher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    pub encryptCipher: *mut crate::Org::BouncyCastle::Crypto::IStreamCipher,
    pub decryptCipher: *mut crate::Org::BouncyCastle::Crypto::IStreamCipher,
    pub writeMac: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsMac,
    pub readMac: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsMac,
    pub usesNonce: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsStreamCipher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsStreamCipher
    => "Org.BouncyCastle.Crypto.Tls"."TlsStreamCipher"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsStreamCipher")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsStreamCipher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsStreamCipher")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsStreamCipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsStreamCipher")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsStreamCipher {
    pub fn CheckMac(
        &mut self,
        seqNo: i64,
        _cordl_type: u8,
        recBuf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        recStart: i32,
        recEnd: i32,
        calcBuf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        calcOff: i32,
        calcLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CheckMac",
                (seqNo, _cordl_type, recBuf, recStart, recEnd, calcBuf, calcOff, calcLen),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DecodeCiphertext(
        &mut self,
        seqNo: i64,
        _cordl_type: u8,
        ciphertext: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DecodeCiphertext", (seqNo, _cordl_type, ciphertext, offset, len))?;
        Ok(__cordl_ret)
    }
    pub fn EncodePlaintext(
        &mut self,
        seqNo: i64,
        _cordl_type: u8,
        plaintext: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("EncodePlaintext", (seqNo, _cordl_type, plaintext, offset, len))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        clientWriteCipher: *mut crate::Org::BouncyCastle::Crypto::IStreamCipher,
        serverWriteCipher: *mut crate::Org::BouncyCastle::Crypto::IStreamCipher,
        clientWriteDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        serverWriteDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        cipherKeySize: i32,
        usesNonce: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    context,
                    clientWriteCipher,
                    serverWriteCipher,
                    clientWriteDigest,
                    serverWriteDigest,
                    cipherKeySize,
                    usesNonce,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn UpdateIV(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IStreamCipher,
        forEncryption: bool,
        seqNo: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIV", (cipher, forEncryption, seqNo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        clientWriteCipher: *mut crate::Org::BouncyCastle::Crypto::IStreamCipher,
        serverWriteCipher: *mut crate::Org::BouncyCastle::Crypto::IStreamCipher,
        clientWriteDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        serverWriteDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        cipherKeySize: i32,
        usesNonce: bool,
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
                    clientWriteDigest,
                    serverWriteDigest,
                    cipherKeySize,
                    usesNonce,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsStreamCipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsStreamCipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
