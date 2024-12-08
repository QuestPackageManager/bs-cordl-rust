#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsMac")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsMac {
    __cordl_parent: crate::System::Object,
    pub context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    pub secret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    pub digestBlockSize: i32,
    pub digestOverhead: i32,
    pub macLength: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsMac")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsMac =>
    "Org.BouncyCastle.Crypto.Tls"."TlsMac"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsMac")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsMac {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsMac")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsMac {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsMac")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsMac {
    pub fn CalculateMacConstantTime(
        &mut self,
        seqNo: i64,
        _cordl_type: u8,
        message: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
        fullLength: i32,
        dummyData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke(
                "CalculateMacConstantTime",
                (seqNo, _cordl_type, message, offset, length, fullLength, dummyData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetDigestBlockCount(
        &mut self,
        inputLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetDigestBlockCount", (inputLength))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateMac(
        &mut self,
        seqNo: i64,
        _cordl_type: u8,
        message: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("CalculateMac", (seqNo, _cordl_type, message, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn Truncate(
        &mut self,
        bs: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Truncate", (bs))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context, digest, key, keyOff, keyLen))?;
        Ok(__cordl_ret)
    }
    pub fn get_Size(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Size", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MacSecret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_MacSecret", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context, digest, key, keyOff, keyLen))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsMac")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsMac {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
