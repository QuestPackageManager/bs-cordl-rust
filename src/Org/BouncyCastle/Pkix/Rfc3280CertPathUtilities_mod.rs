#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct Rfc3280CertPathUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities => "Org.BouncyCastle.Pkix"
    ."Rfc3280CertPathUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
impl crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    pub fn CheckCrl(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        validDate: crate::System::DateTime,
        defaultCRLSignCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        defaultCRLSignKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
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
                    paramsPKIX,
                    cert,
                    validDate,
                    defaultCRLSignCert,
                    defaultCRLSignKey,
                    certStatus,
                    reasonMask,
                    certPathCerts,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCrls(
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        validDate: crate::System::DateTime,
        sign: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        workingPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        certPathCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckCrls",
                (paramsPKIX, cert, validDate, sign, workingPublicKey, certPathCerts),
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
    pub fn PrepareCertB(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Collections::IList>,
        >,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        policyMapping: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PrepareCertB",
                (certPath, index, policyNodes, validPolicyTree, policyMapping),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertA(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertA", (certPath, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertG(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        nameConstraintValidator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertG", (certPath, index, nameConstraintValidator))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertH1(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        explicitPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertH1", (certPath, index, explicitPolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertH2(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        policyMapping: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertH2", (certPath, index, policyMapping))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertH3(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        inhibitAnyPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertH3", (certPath, index, inhibitAnyPolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertI1(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        explicitPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertI1", (certPath, index, explicitPolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertI2(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        policyMapping: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertI2", (certPath, index, policyMapping))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertJ(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        inhibitAnyPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertJ", (certPath, index, inhibitAnyPolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertK(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertK", (certPath, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertL(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        maxPathLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertL", (certPath, index, maxPathLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertM(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        maxPathLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertM", (certPath, index, maxPathLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertN(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertN", (certPath, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertO(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        criticalExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        pathCheckers: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PrepareNextCertO",
                (certPath, index, criticalExtensions, pathCheckers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertA(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        index: i32,
        workingPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        workingIssuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        >,
        sign: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ProcessCertA",
                (certPath, paramsPKIX, index, workingPublicKey, workingIssuerName, sign),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertBC(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        nameConstraintValidator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCertBC", (certPath, index, nameConstraintValidator))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertD(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        acceptablePolicies: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Collections::IList>,
        >,
        inhibitAnyPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ProcessCertD",
                (
                    certPath,
                    index,
                    acceptablePolicies,
                    validPolicyTree,
                    policyNodes,
                    inhibitAnyPolicy,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertE(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCertE", (certPath, index, validPolicyTree))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertF(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        explicitPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCertF", (certPath, index, validPolicyTree, explicitPolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlA1i(
        currentDate: crate::System::DateTime,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlA1i", (currentDate, paramsPKIX, cert, crl))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlA1ii(
        currentDate: crate::System::DateTime,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlA1ii", (currentDate, paramsPKIX, cert, crl))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlB1(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlB1", (dp, cert, crl))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlB2(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlB2", (dp, cert, crl))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlC(
        deltaCRL: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        completeCRL: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlC", (deltaCRL, completeCRL, pkixParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlD(
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::ReasonsMask>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::ReasonsMask,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlD", (crl, dp))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlF(
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        defaultCRLSignCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        defaultCRLSignKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        certPathCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ProcessCrlF",
                (
                    crl,
                    cert,
                    defaultCRLSignCert,
                    defaultCRLSignKey,
                    paramsPKIX,
                    certPathCerts,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlG(
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        keys: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlG", (crl, keys))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlH(
        deltaCrls: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlH", (deltaCrls, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlI(
        validDate: crate::System::DateTime,
        deltacrl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::CertStatus,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlI", (validDate, deltacrl, cert, certStatus, pkixParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlJ(
        validDate: crate::System::DateTime,
        completecrl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certStatus: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::CertStatus>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCrlJ", (validDate, completecrl, cert, certStatus))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrapupCertA(
        explicitPolicy: i32,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WrapupCertA", (explicitPolicy, cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrapupCertB(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        explicitPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WrapupCertB", (certPath, index, explicitPolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrapupCertF(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        pathCheckers: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        criticalExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WrapupCertF", (certPath, index, pathCheckers, criticalExtensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrapupCertG(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        userInitialPolicySet: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        index: i32,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Collections::IList>,
        >,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        acceptablePolicies: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WrapupCertG",
                (
                    certPath,
                    paramsPKIX,
                    userInitialPolicySet,
                    index,
                    policyNodes,
                    validPolicyTree,
                    acceptablePolicies,
                ),
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
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
