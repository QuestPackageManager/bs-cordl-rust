#[cfg(feature = "Org+BouncyCastle+Crypto+Macs+ISO9797Alg3Mac")]
#[repr(C)]
#[derive(Debug)]
pub struct ISO9797Alg3Mac {
    __cordl_parent: crate::System::Object,
    pub mac: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub bufOff: i32,
    pub cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    pub padding: *mut crate::Org::BouncyCastle::Crypto::Paddings::IBlockCipherPadding,
    pub macSize: i32,
    pub lastKey2: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    pub lastKey3: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Macs+ISO9797Alg3Mac")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Macs::ISO9797Alg3Mac
    => "Org.BouncyCastle.Crypto.Macs"."ISO9797Alg3Mac"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Macs+ISO9797Alg3Mac")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Macs::ISO9797Alg3Mac {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Macs+ISO9797Alg3Mac")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Macs::ISO9797Alg3Mac {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Macs+ISO9797Alg3Mac")]
impl crate::Org::BouncyCastle::Crypto::Macs::ISO9797Alg3Mac {
    pub fn GetMacSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMacSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
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
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn BlockUpdate(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BlockUpdate", (input, inOff, len))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        input: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (input))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IBlockCipher0(
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
    pub fn _ctor_IBlockCipherPadding1(
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
    pub fn _ctor_i32_2(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        macSizeInBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, macSizeInBits))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_IBlockCipherPadding3(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        macSizeInBits: i32,
        padding: *mut crate::Org::BouncyCastle::Crypto::Paddings::IBlockCipherPadding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, macSizeInBits, padding))?;
        Ok(__cordl_ret)
    }
    pub fn New_IBlockCipher0(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher))?;
        Ok(__cordl_object)
    }
    pub fn New_IBlockCipherPadding1(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        padding: *mut crate::Org::BouncyCastle::Crypto::Paddings::IBlockCipherPadding,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, padding))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_2(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        macSizeInBits: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, macSizeInBits))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_IBlockCipherPadding3(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        macSizeInBits: i32,
        padding: *mut crate::Org::BouncyCastle::Crypto::Paddings::IBlockCipherPadding,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, macSizeInBits, padding))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Macs+ISO9797Alg3Mac")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Macs::ISO9797Alg3Mac {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
