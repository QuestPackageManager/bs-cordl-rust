#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct RijndaelManagedTransform {
    __cordl_parent: crate::System::Object,
    pub m_cipherMode: crate::System::Security::Cryptography::CipherMode,
    pub m_paddingValue: crate::System::Security::Cryptography::PaddingMode,
    pub m_transformMode: crate::System::Security::Cryptography::RijndaelManagedTransformMode,
    pub m_blockSizeBits: i32,
    pub m_blockSizeBytes: i32,
    pub m_inputBlockSize: i32,
    pub m_outputBlockSize: i32,
    pub m_encryptKeyExpansion: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_decryptKeyExpansion: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_Nr: i32,
    pub m_Nb: i32,
    pub m_Nk: i32,
    pub m_encryptindex: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_decryptindex: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_IV: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_lastBlockBuffer: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_depadBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub m_shiftRegister: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::RijndaelManagedTransform =>
    "System.Security.Cryptography"."RijndaelManagedTransform"
);
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl std::ops::Deref
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl crate::System::Security::Cryptography::RijndaelManagedTransform {
    pub fn Dec(
        &mut self,
        decryptindex: *mut quest_hook::libil2cpp::Il2CppObject,
        decryptKeyExpansion: *mut quest_hook::libil2cpp::Il2CppObject,
        iT: *mut quest_hook::libil2cpp::Il2CppObject,
        iTF: *mut quest_hook::libil2cpp::Il2CppObject,
        work: *mut quest_hook::libil2cpp::Il2CppObject,
        temp: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dec", (decryptindex, decryptKeyExpansion, iT, iTF, work, temp))?;
        Ok(__cordl_ret)
    }
    pub fn DecryptData(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
        outputBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        outputOffset: i32,
        paddingMode: crate::System::Security::Cryptography::PaddingMode,
        fLast: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "DecryptData",
                (
                    inputBuffer,
                    inputOffset,
                    inputCount,
                    outputBuffer,
                    outputOffset,
                    paddingMode,
                    fLast,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
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
    pub fn Enc(
        &mut self,
        encryptindex: *mut quest_hook::libil2cpp::Il2CppObject,
        encryptKeyExpansion: *mut quest_hook::libil2cpp::Il2CppObject,
        T: *mut quest_hook::libil2cpp::Il2CppObject,
        TF: *mut quest_hook::libil2cpp::Il2CppObject,
        work: *mut quest_hook::libil2cpp::Il2CppObject,
        temp: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enc", (encryptindex, encryptKeyExpansion, T, TF, work, temp))?;
        Ok(__cordl_ret)
    }
    pub fn EncryptData(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
        outputBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        outputOffset: i32,
        paddingMode: crate::System::Security::Cryptography::PaddingMode,
        fLast: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "EncryptData",
                (
                    inputBuffer,
                    inputOffset,
                    inputCount,
                    outputBuffer,
                    outputOffset,
                    paddingMode,
                    fLast,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateKeyExpansion(
        &mut self,
        rgbKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateKeyExpansion", (rgbKey))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        rgbKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        mode: crate::System::Security::Cryptography::CipherMode,
        rgbIV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        blockSize: i32,
        feedbackSize: i32,
        PaddingValue: crate::System::Security::Cryptography::PaddingMode,
        transformMode: crate::System::Security::Cryptography::RijndaelManagedTransformMode,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    rgbKey,
                    mode,
                    rgbIV,
                    blockSize,
                    feedbackSize,
                    PaddingValue,
                    transformMode,
                ),
            )?;
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        rgbKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        mode: crate::System::Security::Cryptography::CipherMode,
        rgbIV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        blockSize: i32,
        feedbackSize: i32,
        PaddingValue: crate::System::Security::Cryptography::PaddingMode,
        transformMode: crate::System::Security::Cryptography::RijndaelManagedTransformMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    rgbKey,
                    mode,
                    rgbIV,
                    blockSize,
                    feedbackSize,
                    PaddingValue,
                    transformMode,
                ),
            )?;
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
    pub fn get_InputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InputBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OutputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OutputBlockSize", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
