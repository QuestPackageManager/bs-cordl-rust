#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsEnvelopedHelper";
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
    )]
    pub type CmsAuthenticatedSecureReadable = crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable;
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable"
    )]
    pub type CmsEnvelopedSecureReadable = crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable;
    pub fn BuildRecipientInformationStore(
        recipientInfos: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Set,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::RecipientInformationStore,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Cms::CmsSecureReadable,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::RecipientInformationStore,
                >,
                2usize,
            >("BuildRecipientInformationStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper as
                    quest_hook::libil2cpp::Type > ::class(),
                    "BuildRecipientInformationStore", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::RecipientInformationStore,
        > = unsafe { method.invoke_unchecked((), (recipientInfos, secureReadable))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateAsymmetricCipher(
        &mut self,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBufferedCipher>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::IBufferedCipher,
                >,
                1usize,
            >("CreateAsymmetricCipher")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper as
                    quest_hook::libil2cpp::Type > ::class(), "CreateAsymmetricCipher",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        > = unsafe { method.invoke_unchecked(self, (encryptionOid))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateWrapper(
        &mut self,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IWrapper>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IWrapper>,
                1usize,
            >("CreateWrapper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper as
                    quest_hook::libil2cpp::Type > ::class(), "CreateWrapper", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IWrapper,
        > = unsafe { method.invoke_unchecked(self, (encryptionOid))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAsymmetricEncryptionAlgName(
        &mut self,
        encryptionAlgOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetAsymmetricEncryptionAlgName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetAsymmetricEncryptionAlgName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (encryptionAlgOid))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetKeySize(
        &mut self,
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("GetKeySize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper as
                    quest_hook::libil2cpp::Type > ::class(), "GetKeySize", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (oid))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRfc3211WrapperName(
        &mut self,
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetRfc3211WrapperName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper as
                    quest_hook::libil2cpp::Type > ::class(), "GetRfc3211WrapperName",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (oid))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadRecipientInfo(
        infos: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        info: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Cms::CmsSecureReadable,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ReadRecipientInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper as
                    quest_hook::libil2cpp::Type > ::class(), "ReadRecipientInfo", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (infos, info, secureReadable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algorithm: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    pub readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsEnvelopedHelper/CmsAuthenticatedSecureReadable";
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
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    pub fn GetReadable(
        &mut self,
        sKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
                1usize,
            >("GetReadable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable
                    as quest_hook::libil2cpp::Type > ::class(), "GetReadable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsReadable,
        > = unsafe { method.invoke_unchecked(self, (sKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, readable))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (algorithm, readable))?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                >,
                0usize,
            >("get_Algorithm")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable
                    as quest_hook::libil2cpp::Type > ::class(), "get_Algorithm", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CryptoObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_CryptoObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable
                    as quest_hook::libil2cpp::Type > ::class(), "get_CryptoObject",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl AsRef<crate::Org::BouncyCastle::Cms::CmsSecureReadable>
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::CmsSecureReadable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl AsMut<crate::Org::BouncyCastle::Cms::CmsSecureReadable>
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Cms::CmsSecureReadable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algorithm: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub cipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IBufferedCipher,
    >,
    pub readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsEnvelopedHelper/CmsEnvelopedSecureReadable";
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    pub fn GetReadable(
        &mut self,
        sKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
                1usize,
            >("GetReadable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable
                    as quest_hook::libil2cpp::Type > ::class(), "GetReadable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsReadable,
        > = unsafe { method.invoke_unchecked(self, (sKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, readable))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (algorithm, readable))?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                >,
                0usize,
            >("get_Algorithm")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable
                    as quest_hook::libil2cpp::Type > ::class(), "get_Algorithm", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CryptoObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_CryptoObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable
                    as quest_hook::libil2cpp::Type > ::class(), "get_CryptoObject",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl AsRef<crate::Org::BouncyCastle::Cms::CmsSecureReadable>
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::CmsSecureReadable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl AsMut<crate::Org::BouncyCastle::Cms::CmsSecureReadable>
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Cms::CmsSecureReadable {
        unsafe { std::mem::transmute(self) }
    }
}
