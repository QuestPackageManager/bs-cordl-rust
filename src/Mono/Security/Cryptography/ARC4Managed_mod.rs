#[cfg(feature = "Mono+Security+Cryptography+ARC4Managed")]
#[repr(C)]
#[derive(Debug)]
pub struct ARC4Managed {
    __cordl_parent: crate::Mono::Security::Cryptography::RC4,
    pub key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub state: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub x: u8,
    pub y: u8,
    pub m_disposed: bool,
}
#[cfg(feature = "Mono+Security+Cryptography+ARC4Managed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::ARC4Managed =>
    "Mono.Security.Cryptography"."ARC4Managed"
);
#[cfg(feature = "Mono+Security+Cryptography+ARC4Managed")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::ARC4Managed {
    type Target = crate::Mono::Security::Cryptography::RC4;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+ARC4Managed")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::ARC4Managed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+ARC4Managed")]
impl crate::Mono::Security::Cryptography::ARC4Managed {
    pub fn set_Key(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Key", (value))?;
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
    pub fn CreateDecryptor(
        &mut self,
        rgbKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        rgvIV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::ICryptoTransform,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::ICryptoTransform = __cordl_object
            .invoke("CreateDecryptor", (rgbKey, rgvIV))?;
        Ok(__cordl_ret)
    }
    pub fn get_InputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InputBlockSize", ())?;
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
    pub fn get_OutputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OutputBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn KeySetup(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KeySetup", (key))?;
        Ok(__cordl_ret)
    }
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
    pub fn GenerateIV(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateIV", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateEncryptor(
        &mut self,
        rgbKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        rgvIV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::ICryptoTransform,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::ICryptoTransform = __cordl_object
            .invoke("CreateEncryptor", (rgbKey, rgvIV))?;
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Key", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Security+Cryptography+ARC4Managed")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::ARC4Managed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
