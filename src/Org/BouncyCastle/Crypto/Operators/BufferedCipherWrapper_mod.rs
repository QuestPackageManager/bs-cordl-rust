#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedCipherWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub bufferedCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IBufferedCipher,
    >,
    pub stream: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IO::CipherStream,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper =>
    "Org.BouncyCastle.Crypto.Operators"."BufferedCipherWrapper"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    pub fn GetMaxOutputSize(
        &mut self,
        inputLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMaxOutputSize", (inputLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUpdateOutputSize(
        &mut self,
        inputLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUpdateOutputSize", (inputLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bufferedCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        >,
        source: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bufferedCipher, source))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bufferedCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        >,
        source: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bufferedCipher, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Stream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("get_Stream", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipher>
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+BufferedCipherWrapper")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipher>
for crate::Org::BouncyCastle::Crypto::Operators::BufferedCipherWrapper {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipher {
        unsafe { std::mem::transmute(self) }
    }
}
