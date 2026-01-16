#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampTokenGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub accuracySeconds: i32,
    pub accuracyMillis: i32,
    pub accuracyMicros: i32,
    pub ordering: bool,
    pub tsa: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::GeneralName>,
    pub tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub key: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter>,
    pub cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    pub digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub signedAttr: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::AttributeTable>,
    pub unsignedAttr:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::AttributeTable>,
    pub x509Certs: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
    pub x509Crls: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
unsafe impl quest_hook::libil2cpp::Type for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Tsp";
    const CLASS_NAME: &'static str = "TimeStampTokenGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    pub fn Generate(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Tsp::TimeStampRequest>,
        serialNumber: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        genTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Tsp::TimeStampToken>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Tsp::TimeStampRequest,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            crate::System::DateTime,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Tsp::TimeStampToken,
                        >,
                        3usize,
                    >("Generate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Generate", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Tsp::TimeStampToken> =
            unsafe { cordl_method_info.invoke_unchecked(self, (request, serialNumber, genTime))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_AsymmetricKeyParameter_X509Certificate_Il2CppString_Il2CppString0(
        key: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter>,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, cert, digestOID, tsaPolicyOID))?;
        Ok(__cordl_object.into())
    }
    pub fn New_AttributeTable_AttributeTable1(
        key: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter>,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signedAttr: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::AttributeTable>,
        unsignedAttr: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (key, cert, digestOID, tsaPolicyOID, signedAttr, unsignedAttr),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn SetAccuracyMicros(
        &mut self,
        accuracyMicros: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("SetAccuracyMicros")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetAccuracyMicros",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (accuracyMicros))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetAccuracyMillis(
        &mut self,
        accuracyMillis: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("SetAccuracyMillis")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetAccuracyMillis",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (accuracyMillis))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetAccuracySeconds(
        &mut self,
        accuracySeconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("SetAccuracySeconds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetAccuracySeconds",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (accuracySeconds))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetCertificates(
        &mut self,
        certificates: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::X509::Store::IX509Store,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetCertificates")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetCertificates", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (certificates))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetCrls(
        &mut self,
        crls: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::X509::Store::IX509Store,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetCrls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "SetCrls",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (crls))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetOrdering(
        &mut self,
        ordering: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetOrdering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetOrdering",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (ordering))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetTsa(
        &mut self,
        tsa: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::GeneralName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetTsa")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "SetTsa",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (tsa))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AsymmetricKeyParameter_X509Certificate_Il2CppString_Il2CppString0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter>,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (key, cert, digestOID, tsaPolicyOID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AttributeTable_AttributeTable1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter>,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        digestOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tsaPolicyOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signedAttr: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::AttributeTable>,
        unsignedAttr: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
                        >,
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
                (key, cert, digestOID, tsaPolicyOID, signedAttr, unsignedAttr),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
