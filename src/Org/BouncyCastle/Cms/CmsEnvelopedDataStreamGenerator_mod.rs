#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedDataStreamGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
    pub _originatorInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _unprotectedAttributes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _bufferSize: i32,
    pub _berEncodeRecipientSet: bool,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsEnvelopedDataStreamGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    type Target = crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
    )]
    pub type CmsEnvelopedDataOutputStream = crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream;
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecureRandom1(
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_rand))?;
        Ok(__cordl_object.into())
    }
    pub fn Open_AlgorithmIdentifier_ICipherParameters_Asn1EncodableVector1(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encAlgID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        cipherParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
        recipientInfos: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::ICipherParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        4usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Open",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (outStream, encAlgID, cipherParameters, recipientInfos),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Open_Il2CppString2(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        2usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Open",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info.invoke_unchecked(self, (outStream, encryptionOid))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Open_Il2CppString_CipherKeyGenerator0(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keyGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        3usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Open",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info.invoke_unchecked(self, (outStream, encryptionOid, keyGen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Open_Il2CppString_i32_3(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        3usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Open",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (outStream, encryptionOid, keySize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBerEncodeRecipients(
        &mut self,
        berEncodeRecipientSet: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetBerEncodeRecipients")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBerEncodeRecipients", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (berEncodeRecipientSet))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferSize(
        &mut self,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetBufferSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferSize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (bufferSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom1(
        &mut self,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Security::SecureRandom,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_rand))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerInteger,
                        >,
                        0usize,
                    >("get_Version")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Version", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerInteger,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream,
    pub _outer: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
    >,
    pub _out: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IO::CipherStream,
    >,
    pub _cGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    >,
    pub _envGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    >,
    pub _eiGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    >,
}
#[cfg(
    feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsEnvelopedDataStreamGenerator/CmsEnvelopedDataOutputStream";
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
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Close")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Close",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        outer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
        >,
        outStream: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IO::CipherStream,
        >,
        cGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
        envGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
        eiGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer, outStream, cGen, envGen, eiGen))?;
        Ok(__cordl_object.into())
    }
    pub fn Write(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
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
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Write",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (bytes, off, len))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteByte(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u8),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("WriteByte")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteByte", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        outer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
        >,
        outStream: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IO::CipherStream,
        >,
        cGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
        envGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
        eiGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::IO::CipherStream,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (outer, outStream, cGen, envGen, eiGen))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
