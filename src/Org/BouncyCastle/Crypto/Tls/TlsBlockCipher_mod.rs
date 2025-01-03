#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsBlockCipher")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsBlockCipher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub context: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    >,
    pub randomData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub useExplicitIV: bool,
    pub encryptThenMac: bool,
    pub encryptCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IBlockCipher,
    >,
    pub decryptCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IBlockCipher,
    >,
    pub mWriteMac: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsMac,
    >,
    pub mReadMac: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsMac,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsBlockCipher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher
    => "Org.BouncyCastle.Crypto.Tls"."TlsBlockCipher"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsBlockCipher")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsBlockCipher")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsBlockCipher")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher {
    pub fn CheckPaddingConstantTime(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
        blockSize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CheckPaddingConstantTime", (buf, off, len, blockSize, macSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChooseExtraPadBlocks(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ChooseExtraPadBlocks", (r, max))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn LowestBitSet(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LowestBitSet", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockCipher,
        >,
        clientWriteDigest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        >,
        serverWriteDigest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        >,
        cipherKeySize: i32,
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
                    clientWriteDigest,
                    serverWriteDigest,
                    cipherKeySize,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockCipher,
        >,
        clientWriteDigest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        >,
        serverWriteDigest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        >,
        cipherKeySize: i32,
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
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReadMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsMac>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsMac,
        > = __cordl_object.invoke("get_ReadMac", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WriteMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsMac>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsMac,
        > = __cordl_object.invoke("get_WriteMac", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsBlockCipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsBlockCipher")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>
for crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsCipher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsBlockCipher")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>
for crate::Org::BouncyCastle::Crypto::Tls::TlsBlockCipher {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher {
        unsafe { std::mem::transmute(self) }
    }
}
