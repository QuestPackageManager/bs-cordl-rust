#[cfg(feature = "Org+BouncyCastle+Crypto+Paddings+PaddedBufferedBlockCipher")]
#[repr(C)]
#[derive(Debug)]
pub struct PaddedBufferedBlockCipher {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::BufferedBlockCipher,
    pub padding: *mut crate::Org::BouncyCastle::Crypto::Paddings::IBlockCipherPadding,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Paddings+PaddedBufferedBlockCipher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Paddings::PaddedBufferedBlockCipher =>
    "Org.BouncyCastle.Crypto.Paddings"."PaddedBufferedBlockCipher"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Paddings+PaddedBufferedBlockCipher")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Paddings::PaddedBufferedBlockCipher {
    type Target = crate::Org::BouncyCastle::Crypto::BufferedBlockCipher;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Paddings+PaddedBufferedBlockCipher")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Paddings::PaddedBufferedBlockCipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Paddings+PaddedBufferedBlockCipher")]
impl crate::Org::BouncyCastle::Crypto::Paddings::PaddedBufferedBlockCipher {
    pub fn ProcessByte(
        &mut self,
        input: u8,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessByte", (input, output, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        forEncryption: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forEncryption, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn GetUpdateOutputSize(
        &mut self,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUpdateOutputSize", (length))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessBytes(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        length: i32,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessBytes", (input, inOff, length, output, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IBlockCipherPadding0(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        padding: *mut crate::Org::BouncyCastle::Crypto::Paddings::IBlockCipherPadding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, padding))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IBlockCipher1(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher))?;
        Ok(__cordl_ret)
    }
    pub fn GetOutputSize(&mut self, length: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetOutputSize", (length))?;
        Ok(__cordl_ret)
    }
    pub fn DoFinal(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("DoFinal", (output, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn New_IBlockCipherPadding0(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        padding: *mut crate::Org::BouncyCastle::Crypto::Paddings::IBlockCipherPadding,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, padding))?;
        Ok(__cordl_object)
    }
    pub fn New_IBlockCipher1(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Paddings+PaddedBufferedBlockCipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Paddings::PaddedBufferedBlockCipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
