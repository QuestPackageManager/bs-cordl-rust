#[cfg(feature = "Mono+Security+Cryptography+SymmetricTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct SymmetricTransform {
    __cordl_parent: crate::System::Object,
    pub algo: *mut crate::System::Security::Cryptography::SymmetricAlgorithm,
    pub encrypt: bool,
    pub BlockSizeByte: i32,
    pub temp: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub temp2: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub workBuff: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub workout: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub padmode: crate::System::Security::Cryptography::PaddingMode,
    pub FeedBackByte: i32,
    pub m_disposed: bool,
    pub lastBlock: bool,
    pub _rng: *mut crate::System::Security::Cryptography::RandomNumberGenerator,
}
#[cfg(feature = "Mono+Security+Cryptography+SymmetricTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::SymmetricTransform
    => "Mono.Security.Cryptography"."SymmetricTransform"
);
#[cfg(feature = "Mono+Security+Cryptography+SymmetricTransform")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::SymmetricTransform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+SymmetricTransform")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::SymmetricTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+SymmetricTransform")]
impl crate::Mono::Security::Cryptography::SymmetricTransform {
    pub fn TransformFinalBlock(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("TransformFinalBlock", (inputBuffer, inputOffset, inputCount))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowBadPaddingException(
        &mut self,
        padding: crate::System::Security::Cryptography::PaddingMode,
        length: i32,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowBadPaddingException", (padding, length, position))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn CFB(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CFB", (input, output))?;
        Ok(__cordl_ret)
    }
    pub fn CheckInput(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckInput", (inputBuffer, inputOffset, inputCount))?;
        Ok(__cordl_ret)
    }
    pub fn get_KeepLastBlock(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_KeepLastBlock", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn get_OutputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OutputBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OFB(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OFB", (input, output))?;
        Ok(__cordl_ret)
    }
    pub fn get_InputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InputBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn CBC(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CBC", (input, output))?;
        Ok(__cordl_ret)
    }
    pub fn Transform(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Transform", (input, output))?;
        Ok(__cordl_ret)
    }
    pub fn InternalTransformBlock(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
        outputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outputOffset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "InternalTransformBlock",
                (inputBuffer, inputOffset, inputCount, outputBuffer, outputOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn FinalDecrypt(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("FinalDecrypt", (inputBuffer, inputOffset, inputCount))?;
        Ok(__cordl_ret)
    }
    pub fn ECB(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ECB", (input, output))?;
        Ok(__cordl_ret)
    }
    pub fn CTS(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CTS", (input, output))?;
        Ok(__cordl_ret)
    }
    pub fn FinalEncrypt(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("FinalEncrypt", (inputBuffer, inputOffset, inputCount))?;
        Ok(__cordl_ret)
    }
    pub fn get_CanTransformMultipleBlocks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CanTransformMultipleBlocks", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        symmAlgo: *mut crate::System::Security::Cryptography::SymmetricAlgorithm,
        encryption: bool,
        rgbIV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (symmAlgo, encryption, rgbIV))?;
        Ok(__cordl_ret)
    }
    pub fn Random(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Random", (buffer, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn TransformBlock(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
        outputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outputOffset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "TransformBlock",
                (inputBuffer, inputOffset, inputCount, outputBuffer, outputOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        symmAlgo: *mut crate::System::Security::Cryptography::SymmetricAlgorithm,
        encryption: bool,
        rgbIV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (symmAlgo, encryption, rgbIV))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Security+Cryptography+SymmetricTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::SymmetricTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
