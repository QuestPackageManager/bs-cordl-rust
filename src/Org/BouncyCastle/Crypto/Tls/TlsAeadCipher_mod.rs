#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsAeadCipher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub context: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>,
    pub macSize: i32,
    pub record_iv_length: i32,
    pub encryptCipher:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher>,
    pub decryptCipher:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher>,
    pub encryptImplicitNonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub decryptImplicitNonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub nonceMode: i32,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
unsafe impl quest_hook::libil2cpp::Type for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "TlsAeadCipher";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    pub const NONCE_DRAFT_CHACHA20_POLY1305: i32 = 2i32;
    pub const NONCE_RFC5288: i32 = 1i32;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i64,
                        u8,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>, 5usize>(
                        "DecodeCiphertext",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DecodeCiphertext",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (seqNo, _cordl_type, ciphertext, offset, len))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i64,
                        u8,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>, 5usize>(
                        "EncodePlaintext",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EncodePlaintext",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (seqNo, _cordl_type, plaintext, offset, len))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i64, u8, i32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        3usize,
                    >("GetAdditionalData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAdditionalData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>> =
            unsafe { cordl_method_info.invoke_unchecked(self, (seqNo, _cordl_type, len))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPlaintextLimit(
        &mut self,
        ciphertextLimit: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("GetPlaintextLimit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPlaintextLimit",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (ciphertextLimit))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_TlsContext_IAeadBlockCipher_IAeadBlockCipher_i32_i32_0(
        context: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        cipherKeySize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                context,
                clientWriteCipher,
                serverWriteCipher,
                cipherKeySize,
                macSize,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        context: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        cipherKeySize: i32,
        macSize: i32,
        nonceMode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                context,
                clientWriteCipher,
                serverWriteCipher,
                cipherKeySize,
                macSize,
                nonceMode,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_TlsContext_IAeadBlockCipher_IAeadBlockCipher_i32_i32_0(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        cipherKeySize: i32,
        macSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
                        >,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    context,
                    clientWriteCipher,
                    serverWriteCipher,
                    cipherKeySize,
                    macSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>,
        clientWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        serverWriteCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
        >,
        cipherKeySize: i32,
        macSize: i32,
        nonceMode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher,
                        >,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    context,
                    clientWriteCipher,
                    serverWriteCipher,
                    cipherKeySize,
                    macSize,
                    nonceMode,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>
    for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsCipher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsAeadCipher")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>
    for crate::Org::BouncyCastle::Crypto::Tls::TlsAeadCipher
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher {
        unsafe { std::mem::transmute(self) }
    }
}
