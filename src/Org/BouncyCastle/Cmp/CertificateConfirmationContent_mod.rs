#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContent")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateConfirmationContent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub digestAlgFinder: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
    >,
    pub content: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cmp";
    const CLASS_NAME: &'static str = "CertificateConfirmationContent";
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
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContent")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContent")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContent")]
impl crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent {
    pub fn GetStatusMessages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cmp::CertificateStatus,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::Org::BouncyCastle::Cmp::CertificateStatus,
                                >,
                            >,
                        >,
                        0usize,
                    >("GetStatusMessages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetStatusMessages", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cmp::CertificateStatus,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New_CertConfirmContent0(
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultDigestAlgorithmIdentifierFinder1(
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
        >,
        digestAlgFinder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content, digestAlgFinder))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Structure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
                        >,
                        0usize,
                    >("ToAsn1Structure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToAsn1Structure", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CertConfirmContent0(
        &mut self,
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (content))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultDigestAlgorithmIdentifierFinder1(
        &mut self,
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
        >,
        digestAlgFinder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::Cmp::CertConfirmContent,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (content, digestAlgFinder))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
