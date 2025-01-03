#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3281CertPathUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct Rfc3281CertPathUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3281CertPathUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkix::Rfc3281CertPathUtilities => "Org.BouncyCastle.Pkix"
    ."Rfc3281CertPathUtilities"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdditionalChecks", (attrCert, pkixParams))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckCrl",
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
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckCrls",
                (attrCert, paramsPKIX, issuerCert, validDate, certPathCerts),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessAttrCert1", (attrCert, pkixParams))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessAttrCert2", (certPath, pkixParams))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessAttrCert3", (acIssuerCert, pkixParams))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessAttrCert4", (acIssuerCert, pkixParams))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessAttrCert5", (attrCert, pkixParams))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ProcessAttrCert7",
                (attrCert, certPath, holderCertPath, pkixParams),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
