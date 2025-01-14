#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3281CertPathUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct Rfc3281CertPathUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3281CertPathUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Pkix::Rfc3281CertPathUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Pkix";
    const CLASS_NAME: &'static str = "Rfc3281CertPathUtilities";
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
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3281CertPathUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::Rfc3281CertPathUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3281CertPathUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::Rfc3281CertPathUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3281CertPathUtilities")]
impl crate::Org::BouncyCastle::Pkix::Rfc3281CertPathUtilities {
    pub fn AdditionalChecks(
        attrCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AdditionalChecks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AdditionalChecks", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (attrCert, pkixParams))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCrl(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        attrCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        validDate: crate::System::DateTime,
        issuerCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::CertStatus,
        >,
        reasonMask: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::ReasonsMask,
        >,
        certPathCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                    crate::System::DateTime,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::X509Certificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::CertStatus,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::ReasonsMask,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("CheckCrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckCrl", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        dp,
                        attrCert,
                        paramsPKIX,
                        validDate,
                        issuerCert,
                        certStatus,
                        reasonMask,
                        certPathCerts,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCrls(
        attrCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        issuerCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        validDate: crate::System::DateTime,
        certPathCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::X509Certificate,
                    >,
                    crate::System::DateTime,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("CheckCrls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckCrls", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (attrCert, paramsPKIX, issuerCert, validDate, certPathCerts),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessAttrCert1(
        attrCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixCertPath>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixCertPath>,
                2usize,
            >("ProcessAttrCert1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessAttrCert1", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        > = unsafe { method.invoke_unchecked((), (attrCert, pkixParams)) };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessAttrCert2(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixCertPath,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult,
                >,
                2usize,
            >("ProcessAttrCert2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessAttrCert2", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult,
        > = unsafe { method.invoke_unchecked((), (certPath, pkixParams)) };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessAttrCert3(
        acIssuerCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::X509Certificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessAttrCert3")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessAttrCert3", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (acIssuerCert, pkixParams))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessAttrCert4(
        acIssuerCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::X509Certificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessAttrCert4")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessAttrCert4", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (acIssuerCert, pkixParams))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessAttrCert5(
        attrCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessAttrCert5")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessAttrCert5", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (attrCert, pkixParams))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessAttrCert7(
        attrCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        holderCertPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixCertPath,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixCertPath,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixParameters,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ProcessAttrCert7")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessAttrCert7", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (attrCert, certPath, holderCertPath, pkixParams))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3281CertPathUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::Rfc3281CertPathUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
