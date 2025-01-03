#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Chacha20Poly1305")]
#[repr(C)]
#[derive(Debug)]
pub struct Chacha20Poly1305 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub context: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    >,
    pub encryptCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Engines::ChaCha7539Engine,
    >,
    pub decryptCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Engines::ChaCha7539Engine,
    >,
    pub encryptIV: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub decryptIV: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Chacha20Poly1305")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::Chacha20Poly1305
    => "Org.BouncyCastle.Crypto.Tls"."Chacha20Poly1305"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Chacha20Poly1305")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::Chacha20Poly1305 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Chacha20Poly1305")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::Chacha20Poly1305 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Chacha20Poly1305")]
impl crate::Org::BouncyCastle::Crypto::Tls::Chacha20Poly1305 {
    pub fn CalculateNonce(
        &mut self,
        seqNo: i64,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("CalculateNonce", (seqNo, iv))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateRecordMac(
        &mut self,
        macKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
        additionalData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
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
            .invoke("CalculateRecordMac", (macKey, additionalData, buf, off, len))?;
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
    pub fn GenerateRecordMacKey(
        &mut self,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IStreamCipher,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = __cordl_object.invoke("GenerateRecordMacKey", (cipher))?;
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
    pub fn InitRecord(
        &mut self,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IStreamCipher,
        >,
        forEncryption: bool,
        seqNo: i64,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = __cordl_object.invoke("InitRecord", (cipher, forEncryption, seqNo, iv))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context))?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateRecordMacLength(
        &mut self,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRecordMacLength", (mac, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRecordMacText(
        &mut self,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRecordMacText", (mac, buf, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Chacha20Poly1305")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::Chacha20Poly1305 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Chacha20Poly1305")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>
for crate::Org::BouncyCastle::Crypto::Tls::Chacha20Poly1305 {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsCipher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Chacha20Poly1305")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>
for crate::Org::BouncyCastle::Crypto::Tls::Chacha20Poly1305 {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher {
        unsafe { std::mem::transmute(self) }
    }
}
