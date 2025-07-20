#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct RijndaelManagedTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_cipherMode: crate::System::Security::Cryptography::CipherMode,
    pub m_paddingValue: crate::System::Security::Cryptography::PaddingMode,
    pub m_transformMode: crate::System::Security::Cryptography::RijndaelManagedTransformMode,
    pub m_blockSizeBits: i32,
    pub m_blockSizeBytes: i32,
    pub m_inputBlockSize: i32,
    pub m_outputBlockSize: i32,
    pub m_encryptKeyExpansion: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub m_decryptKeyExpansion: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub m_Nr: i32,
    pub m_Nb: i32,
    pub m_Nk: i32,
    pub m_encryptindex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub m_decryptindex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub m_IV: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub m_lastBlockBuffer: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub m_depadBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub m_shiftRegister: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography";
    const CLASS_NAME: &'static str = "RijndaelManagedTransform";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl std::ops::Deref
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl crate::System::Security::Cryptography::RijndaelManagedTransform {
    pub fn Dec(
        &mut self,
        decryptindex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        decryptKeyExpansion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        iT: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        iTF: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        work: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        temp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("Dec")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dec",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (decryptindex, decryptKeyExpansion, iT, iTF, work, temp),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DecryptData(
        &mut self,
        inputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inputOffset: i32,
        inputCount: i32,
        outputBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        outputOffset: i32,
        paddingMode: crate::System::Security::Cryptography::PaddingMode,
        fLast: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<u8>,
                                >,
                            >,
                            i32,
                            crate::System::Security::Cryptography::PaddingMode,
                            bool,
                        ),
                        i32,
                        7usize,
                    >("DecryptData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DecryptData", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        inputBuffer,
                        inputOffset,
                        inputCount,
                        outputBuffer,
                        outputOffset,
                        paddingMode,
                        fLast,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (disposing))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Enc(
        &mut self,
        encryptindex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        encryptKeyExpansion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        T: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        TF: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        work: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        temp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("Enc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Enc",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (encryptindex, encryptKeyExpansion, T, TF, work, temp),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncryptData(
        &mut self,
        inputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inputOffset: i32,
        inputCount: i32,
        outputBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        outputOffset: i32,
        paddingMode: crate::System::Security::Cryptography::PaddingMode,
        fLast: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<u8>,
                                >,
                            >,
                            i32,
                            crate::System::Security::Cryptography::PaddingMode,
                            bool,
                        ),
                        i32,
                        7usize,
                    >("EncryptData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EncryptData", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        inputBuffer,
                        inputOffset,
                        inputCount,
                        outputBuffer,
                        outputOffset,
                        paddingMode,
                        fLast,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateKeyExpansion(
        &mut self,
        rgbKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GenerateKeyExpansion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateKeyExpansion", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rgbKey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MulX(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("MulX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "MulX",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        rgbKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mode: crate::System::Security::Cryptography::CipherMode,
        rgbIV: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        blockSize: i32,
        feedbackSize: i32,
        PaddingValue: crate::System::Security::Cryptography::PaddingMode,
        transformMode: crate::System::Security::Cryptography::RijndaelManagedTransformMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubWord(a: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("SubWord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "SubWord",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (a))? };
        Ok(__cordl_ret.into())
    }
    pub fn TransformBlock(
        &mut self,
        inputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inputOffset: i32,
        inputCount: i32,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outputOffset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                        ),
                        i32,
                        5usize,
                    >("TransformBlock")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TransformBlock", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (inputBuffer, inputOffset, inputCount, outputBuffer, outputOffset),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransformFinalBlock(
        &mut self,
        inputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inputOffset: i32,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        3usize,
                    >("TransformFinalBlock")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TransformFinalBlock", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (inputBuffer, inputOffset, inputCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rgbKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mode: crate::System::Security::Cryptography::CipherMode,
        rgbIV: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        blockSize: i32,
        feedbackSize: i32,
        PaddingValue: crate::System::Security::Cryptography::PaddingMode,
        transformMode: crate::System::Security::Cryptography::RijndaelManagedTransformMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            crate::System::Security::Cryptography::CipherMode,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                            crate::System::Security::Cryptography::PaddingMode,
                            crate::System::Security::Cryptography::RijndaelManagedTransformMode,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        rgbKey,
                        mode,
                        rgbIV,
                        blockSize,
                        feedbackSize,
                        PaddingValue,
                        transformMode,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CanTransformMultipleBlocks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_CanTransformMultipleBlocks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CanTransformMultipleBlocks", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_InputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_InputBlockSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_InputBlockSize", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_OutputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_OutputBlockSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_OutputBlockSize", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn rot1(val: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("rot1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "rot1",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (val))? };
        Ok(__cordl_ret.into())
    }
    pub fn rot2(val: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("rot2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "rot2",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (val))? };
        Ok(__cordl_ret.into())
    }
    pub fn rot3(val: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("rot3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "rot3",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (val))? };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl AsRef<crate::System::IDisposable>
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl AsMut<crate::System::IDisposable>
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl AsRef<crate::System::Security::Cryptography::ICryptoTransform>
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    fn as_ref(&self) -> &crate::System::Security::Cryptography::ICryptoTransform {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransform")]
impl AsMut<crate::System::Security::Cryptography::ICryptoTransform>
for crate::System::Security::Cryptography::RijndaelManagedTransform {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Security::Cryptography::ICryptoTransform {
        unsafe { std::mem::transmute(self) }
    }
}
