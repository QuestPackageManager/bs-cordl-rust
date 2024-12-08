#[cfg(feature = "Org+BouncyCastle+Pkix+PkixParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixParameters {
    __cordl_parent: crate::System::Object,
    pub trustAnchors: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub date: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    pub certPathCheckers: *mut crate::System::Collections::IList,
    pub revocationEnabled: bool,
    pub initialPolicies: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub explicitPolicyRequired: bool,
    pub anyPolicyInhibited: bool,
    pub policyMappingInhibited: bool,
    pub policyQualifiersRejected: bool,
    pub certSelector: *mut crate::Org::BouncyCastle::X509::Store::IX509Selector,
    pub stores: *mut crate::System::Collections::IList,
    pub selector: *mut crate::Org::BouncyCastle::X509::Store::IX509Selector,
    pub additionalLocationsEnabled: bool,
    pub additionalStores: *mut crate::System::Collections::IList,
    pub trustedACIssuers: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub necessaryACAttributes: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub prohibitedACAttributes: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub attrCertCheckers: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub validityModel: i32,
    pub useDeltas: bool,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::PkixParameters =>
    "Org.BouncyCastle.Pkix"."PkixParameters"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixParameters")]
impl crate::Org::BouncyCastle::Pkix::PkixParameters {
    pub const ChainValidityModel: i32 = 1i32;
    pub const PkixValidityModel: i32 = 0i32;
    pub fn set_IsExplicitPolicyRequired(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsExplicitPolicyRequired", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ValidityModel(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValidityModel", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AddCertPathChecker(
        &mut self,
        checker: *mut crate::Org::BouncyCastle::Pkix::PkixCertPathChecker,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCertPathChecker", (checker))?;
        Ok(__cordl_ret)
    }
    pub fn AddStore(
        &mut self,
        store: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStore", (store))?;
        Ok(__cordl_ret)
    }
    pub fn GetTargetConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::IX509Selector,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::IX509Selector = __cordl_object
            .invoke("GetTargetConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsPolicyQualifiersRejected(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsPolicyQualifiersRejected", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetAttrCertCheckers(
        &mut self,
        attrCertCheckers: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAttrCertCheckers", (attrCertCheckers))?;
        Ok(__cordl_ret)
    }
    pub fn GetNecessaryACAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetNecessaryACAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCertPathCheckers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("GetCertPathCheckers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsUseDeltasEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUseDeltasEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsUseDeltasEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsUseDeltasEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Date(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Date", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetStores(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("GetStores", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsRevocationEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsRevocationEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsExplicitPolicyRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsExplicitPolicyRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTrustAnchors(
        &mut self,
        tas: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrustAnchors", (tas))?;
        Ok(__cordl_ret)
    }
    pub fn AddAdditionalStore(
        &mut self,
        store: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAdditionalStore", (store))?;
        Ok(__cordl_ret)
    }
    pub fn SetAdditionalLocationsEnabled(
        &mut self,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAdditionalLocationsEnabled", (enabled))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsRevocationEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsRevocationEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetTargetCertConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector = __cordl_object
            .invoke("GetTargetCertConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsPolicyMappingInhibited(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsPolicyMappingInhibited", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetCertPathCheckers(
        &mut self,
        checkers: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCertPathCheckers", (checkers))?;
        Ok(__cordl_ret)
    }
    pub fn SetNecessaryACAttributes(
        &mut self,
        necessaryACAttributes: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNecessaryACAttributes", (necessaryACAttributes))?;
        Ok(__cordl_ret)
    }
    pub fn GetTrustAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetTrustAnchors", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTrustedACIssuers(
        &mut self,
        trustedACIssuers: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrustedACIssuers", (trustedACIssuers))?;
        Ok(__cordl_ret)
    }
    pub fn SetProhibitedACAttributes(
        &mut self,
        prohibitedACAttributes: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProhibitedACAttributes", (prohibitedACAttributes))?;
        Ok(__cordl_ret)
    }
    pub fn SetTargetCertConstraints(
        &mut self,
        selector: *mut crate::Org::BouncyCastle::X509::Store::IX509Selector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetCertConstraints", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn SetParams(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Pkix::PkixParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParams", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAdditionalLocationsEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsAdditionalLocationsEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAnyPolicyInhibited(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAnyPolicyInhibited", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPolicyMappingInhibited(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsPolicyMappingInhibited", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValidityModel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ValidityModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Date(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject = __cordl_object
            .invoke("get_Date", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetStores(
        &mut self,
        stores: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStores", (stores))?;
        Ok(__cordl_ret)
    }
    pub fn SetInitialPolicies(
        &mut self,
        initialPolicies: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInitialPolicies", (initialPolicies))?;
        Ok(__cordl_ret)
    }
    pub fn GetAdditionalStores(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("GetAdditionalStores", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTrustedACIssuers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetTrustedACIssuers", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsAnyPolicyInhibited(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsAnyPolicyInhibited", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttrCertCheckers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetAttrCertCheckers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPolicyQualifiersRejected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsPolicyQualifiersRejected", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInitialPolicies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetInitialPolicies", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        trustAnchors: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trustAnchors))?;
        Ok(__cordl_ret)
    }
    pub fn SetTargetConstraints(
        &mut self,
        selector: *mut crate::Org::BouncyCastle::X509::Store::IX509Selector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetConstraints", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn GetProhibitedACAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetProhibitedACAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        trustAnchors: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trustAnchors))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
