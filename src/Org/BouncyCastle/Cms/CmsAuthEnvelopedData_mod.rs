#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAuthEnvelopedData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub recipientInfoStore: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::RecipientInformationStore,
    >,
    pub contentInfo: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    >,
    pub originator: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
    >,
    pub authEncAlg: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub authAttrs: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
    pub mac: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub unauthAttrs: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsAuthEnvelopedData";
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
impl crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable"
    )]
    pub type AuthEnvelopedSecureReadable = crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable;
    pub fn New_ContentInfo2(
        contentInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray0(
        authEnvData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (authEnvData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream1(
        authEnvData: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (authEnvData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ContentInfo2(
        &mut self,
        contentInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
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
            cordl_method_info.invoke_unchecked(self, (contentInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        authEnvData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
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
            cordl_method_info.invoke_unchecked(self, (authEnvData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream1(
        &mut self,
        authEnvData: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
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
            cordl_method_info.invoke_unchecked(self, (authEnvData))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub parent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsAuthEnvelopedData/AuthEnvelopedSecureReadable";
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    pub fn GetReadable(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Cms::CmsReadable,
                        >,
                        1usize,
                    >("GetReadable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetReadable", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsReadable,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (key))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData,
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
            cordl_method_info.invoke_unchecked(self, (parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                        >,
                        0usize,
                    >("get_Algorithm")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Algorithm", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CryptoObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_CryptoObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CryptoObject", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl AsRef<crate::Org::BouncyCastle::Cms::CmsSecureReadable>
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::CmsSecureReadable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthEnvelopedData+AuthEnvelopedSecureReadable")]
impl AsMut<crate::Org::BouncyCastle::Cms::CmsSecureReadable>
for crate::Org::BouncyCastle::Cms::CmsAuthEnvelopedData_AuthEnvelopedSecureReadable {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Cms::CmsSecureReadable {
        unsafe { std::mem::transmute(self) }
    }
}
