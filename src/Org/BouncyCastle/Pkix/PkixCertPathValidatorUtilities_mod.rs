#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixCertPathValidatorUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkix::PkixCertPathValidatorUtilities => "Org.BouncyCastle.Pkix"
    ."PkixCertPathValidatorUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorUtilities")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorUtilities")]
impl crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorUtilities {
    pub fn AddAdditionalStoreFromLocation(
        location: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddAdditionalStoreFromLocation", (location, pkixParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddAdditionalStoresFromAltNames(
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddAdditionalStoresFromAltNames", (cert, pkixParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddAdditionalStoresFromCrlDistributionPoint(
        crldp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::CrlDistPoint,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddAdditionalStoresFromCrlDistributionPoint", (crldp, pkixParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindCertificates_X509AttrCertStoreSelector1(
        certSelect: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509AttrCertStoreSelector,
        >,
        certStores: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindCertificates", (certSelect, certStores))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindCertificates_X509CertStoreSelector0(
        certSelect: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
        certStores: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindCertificates", (certSelect, certStores))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIssuerCerts(
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIssuerCerts", (cert, pkixParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindTrustAnchor(
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        trustAnchors: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::TrustAnchor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::TrustAnchor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindTrustAnchor", (cert, trustAnchors))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAlgorithmIdentifier(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAlgorithmIdentifier", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCertStatus(
        validDate: crate::System::DateTime,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certStatus: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::CertStatus>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCertStatus", (validDate, crl, cert, certStatus))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCompleteCrls(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        currentDate: crate::System::DateTime,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCompleteCrls", (dp, cert, currentDate, paramsPKIX))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCrlIssuersFromDistributionPoint(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        issuerPrincipals: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        >,
        selector: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCrlIssuersFromDistributionPoint",
                (dp, issuerPrincipals, selector, pkixParams),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeltaCrls(
        currentDate: crate::System::DateTime,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        completeCRL: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDeltaCrls", (currentDate, paramsPKIX, completeCRL))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExtensionValue(
        ext: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::IX509Extension>,
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExtensionValue", (ext, oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIssuerPrincipal(
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIssuerPrincipal", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextWorkingKey(
        certs: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextWorkingKey", (certs, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQualifierSet(
        qualifiers: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Sequence,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQualifierSet", (qualifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSerialNumber(
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSerialNumber", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValidCertDateFromValidityModel(
        paramsPkix: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValidCertDateFromValidityModel", (paramsPkix, certPath, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValidDate(
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValidDate", (paramsPKIX))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAnyPolicy(
        policySet: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAnyPolicy", (policySet))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIssuerTrustAnchor(
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        trustAnchors: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIssuerTrustAnchor", (cert, trustAnchors))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSelfIssued(
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSelfIssued", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareNextCertB1(
        i: i32,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        id_p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        m_idp: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertB1", (i, policyNodes, id_p, m_idp, cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertB2(
        i: i32,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        id_p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNextCertB2", (i, policyNodes, id_p, validPolicyTree))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertD1i(
        index: i32,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        pOid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        pq: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCertD1i", (index, policyNodes, pOid, pq))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertD1ii(
        index: i32,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        _poid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        _pq: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessCertD1ii", (index, policyNodes, _poid, _pq))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePolicyNode(
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        _node: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemovePolicyNode", (validPolicyTree, policyNodes, _node))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePolicyNodeRecurse(
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        _node: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemovePolicyNodeRecurse", (policyNodes, _node))?;
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
    pub fn isDeltaCrl(
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isDeltaCrl", (crl))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
