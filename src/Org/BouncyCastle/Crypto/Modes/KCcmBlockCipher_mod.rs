#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+KCcmBlockCipher")]
#[repr(C)]
#[derive(Debug)]
pub struct KCcmBlockCipher {
    __cordl_parent: crate::System::Object,
    pub engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    pub macSize: i32,
    pub forEncryption: bool,
    pub initialAssociatedText: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mac: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub macBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub G1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub s: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub counter: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub associatedText: *mut crate::System::IO::MemoryStream,
    pub data: *mut crate::System::IO::MemoryStream,
    pub Nb_: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+KCcmBlockCipher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Modes::KCcmBlockCipher =>
    "Org.BouncyCastle.Crypto.Modes"."KCcmBlockCipher"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+KCcmBlockCipher")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Modes::KCcmBlockCipher {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+KCcmBlockCipher")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Modes::KCcmBlockCipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+KCcmBlockCipher")]
impl crate::Org::BouncyCastle::Crypto::Modes::KCcmBlockCipher {
    pub fn CalculateMac(
        &mut self,
        authText: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        authOff: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateMac", (authText, authOff, len))?;
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
    pub fn GetBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetMac", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOutputSize(&mut self, len: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetOutputSize", (len))?;
        Ok(__cordl_ret)
    }
    pub fn GetUnderlyingCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher = __cordl_object
            .invoke("GetUnderlyingCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUpdateOutputSize(
        &mut self,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUpdateOutputSize", (len))?;
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
    pub fn New_IBlockCipher0(
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (engine))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        Nb: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (engine, Nb))?;
        Ok(__cordl_object)
    }
    pub fn ProcessAAD(
        &mut self,
        assocText: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        assocOff: i32,
        assocLen: i32,
        dataLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAAD", (assocText, assocOff, assocLen, dataLen))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAadByte(
        &mut self,
        input: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAadByte", (input))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAadBytes(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAadBytes", (input, inOff, len))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessBlock(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        len: i32,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessBlock", (input, inOff, len, output, outOff))?;
        Ok(__cordl_ret)
    }
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
    pub fn ProcessBytes(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessBytes", (input, inOff, inLen, output, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPacket(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        len: i32,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessPacket", (input, inOff, len, output, outOff))?;
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
    pub fn _ctor_IBlockCipher0(
        &mut self,
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (engine))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        Nb: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (engine, Nb))?;
        Ok(__cordl_ret)
    }
    pub fn getFlag(
        &mut self,
        authTextPresents: bool,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object
            .invoke("getFlag", (authTextPresents, macSize))?;
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
    pub fn intToBytes(
        &mut self,
        num: i32,
        outBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("intToBytes", (num, outBytes, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn setNb(
        &mut self,
        Nb: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("setNb", (Nb))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+KCcmBlockCipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Modes::KCcmBlockCipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}