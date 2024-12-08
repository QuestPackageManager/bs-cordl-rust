#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixNameConstraintValidator {
    __cordl_parent: crate::System::Object,
    pub excludedSubtreesDN: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub excludedSubtreesDNS: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub excludedSubtreesEmail: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub excludedSubtreesURI: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub excludedSubtreesIP: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub excludedSubtreesOtherName: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub permittedSubtreesDN: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub permittedSubtreesDNS: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub permittedSubtreesEmail: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub permittedSubtreesURI: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub permittedSubtreesIP: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub permittedSubtreesOtherName: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkix::PkixNameConstraintValidator => "Org.BouncyCastle.Pkix"
    ."PkixNameConstraintValidator"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidator")]
impl crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator {
    pub fn ExtractIPsAndSubnetMasks(
        &mut self,
        ipWithSubmask1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ipWithSubmask2: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object
            .invoke("ExtractIPsAndSubnetMasks", (ipWithSubmask1, ipWithSubmask2))?;
        Ok(__cordl_ret)
    }
    pub fn UnionIP(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        ip: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("UnionIP", (excluded, ip))?;
        Ok(__cordl_ret)
    }
    pub fn UnionDN(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        dn: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("UnionDN", (excluded, dn))?;
        Ok(__cordl_ret)
    }
    pub fn IsDnsConstrained_String0(
        &mut self,
        constraint: *mut crate::System::String,
        dns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDnsConstrained", (constraint, dns))?;
        Ok(__cordl_ret)
    }
    pub fn IsDnsConstrained_ISet1(
        &mut self,
        constraints: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        dns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDnsConstrained", (constraints, dns))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectDns(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        dnss: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("IntersectDns", (permitted, dnss))?;
        Ok(__cordl_ret)
    }
    pub fn IpWithSubnetMask(
        &mut self,
        ip: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        subnetMask: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("IpWithSubnetMask", (ip, subnetMask))?;
        Ok(__cordl_ret)
    }
    pub fn UnionIPRange(
        &mut self,
        ipWithSubmask1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ipWithSubmask2: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("UnionIPRange", (ipWithSubmask1, ipWithSubmask2))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectIPRange(
        &mut self,
        ipWithSubmask1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ipWithSubmask2: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("IntersectIPRange", (ipWithSubmask1, ipWithSubmask2))?;
        Ok(__cordl_ret)
    }
    pub fn CheckExcludedOtherName(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        name: *mut crate::Org::BouncyCastle::Asn1::X509::OtherName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckExcludedOtherName", (excluded, name))?;
        Ok(__cordl_ret)
    }
    pub fn CheckExcludedEmail(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        email: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckExcludedEmail", (excluded, email))?;
        Ok(__cordl_ret)
    }
    pub fn CheckPermittedDns(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        dns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckPermittedDns", (permitted, dns))?;
        Ok(__cordl_ret)
    }
    pub fn unionURI(
        &mut self,
        email1: *mut crate::System::String,
        email2: *mut crate::System::String,
        _cordl_union: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("unionURI", (email1, email2, _cordl_union))?;
        Ok(__cordl_ret)
    }
    pub fn CheckExcludedUri(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckExcludedUri", (excluded, uri))?;
        Ok(__cordl_ret)
    }
    pub fn UnionDns(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        dns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("UnionDns", (excluded, dns))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectUri_ISet_ISet0(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        uris: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("IntersectUri", (permitted, uris))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectUri_String_String_ISet1(
        &mut self,
        email1: *mut crate::System::String,
        email2: *mut crate::System::String,
        intersect: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IntersectUri", (email1, email2, intersect))?;
        Ok(__cordl_ret)
    }
    pub fn CheckPermittedDirectory(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        directory: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckPermittedDirectory", (permitted, directory))?;
        Ok(__cordl_ret)
    }
    pub fn WithinDomain(
        &mut self,
        testDomain: *mut crate::System::String,
        domain: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WithinDomain", (testDomain, domain))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectPermittedSubtree(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IntersectPermittedSubtree", (permitted))?;
        Ok(__cordl_ret)
    }
    pub fn IsDirectoryConstrained(
        &mut self,
        constraints: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        directory: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDirectoryConstrained", (constraints, directory))?;
        Ok(__cordl_ret)
    }
    pub fn AddExcludedSubtree(
        &mut self,
        subtree: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralSubtree,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExcludedSubtree", (subtree))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectDN(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        dns: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("IntersectDN", (permitted, dns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn HashCollection(
        &mut self,
        c: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("HashCollection", (c))?;
        Ok(__cordl_ret)
    }
    pub fn checkPermitted(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("checkPermitted", (name))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectOtherName_ISet_ISet0(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        otherNames: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("IntersectOtherName", (permitted, otherNames))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectOtherName_OtherName_OtherName_ISet1(
        &mut self,
        otherName1: *mut crate::Org::BouncyCastle::Asn1::X509::OtherName,
        otherName2: *mut crate::Org::BouncyCastle::Asn1::X509::OtherName,
        intersect: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IntersectOtherName", (otherName1, otherName2, intersect))?;
        Ok(__cordl_ret)
    }
    pub fn CheckExcludedDirectory(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        directory: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckExcludedDirectory", (excluded, directory))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectEmail_ISet_ISet0(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        emails: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("IntersectEmail", (permitted, emails))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectEmail_String_String_ISet1(
        &mut self,
        email1: *mut crate::System::String,
        email2: *mut crate::System::String,
        intersect: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IntersectEmail", (email1, email2, intersect))?;
        Ok(__cordl_ret)
    }
    pub fn CheckExcludedIP(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        ip: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckExcludedIP", (excluded, ip))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (o))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractNameAsString(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ExtractNameAsString", (name))?;
        Ok(__cordl_ret)
    }
    pub fn CollectionsAreEqual(
        &mut self,
        coll1: *mut crate::System::Collections::ICollection,
        coll2: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CollectionsAreEqual", (coll1, coll2))?;
        Ok(__cordl_ret)
    }
    pub fn StringifyIPCollection(
        &mut self,
        ips: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("StringifyIPCollection", (ips))?;
        Ok(__cordl_ret)
    }
    pub fn StringifyOtherNameCollection(
        &mut self,
        otherNames: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("StringifyOtherNameCollection", (otherNames))?;
        Ok(__cordl_ret)
    }
    pub fn UnionOtherName(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        otherName: *mut crate::Org::BouncyCastle::Asn1::X509::OtherName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("UnionOtherName", (permitted, otherName))?;
        Ok(__cordl_ret)
    }
    pub fn UnionEmail_ISet0(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        email: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("UnionEmail", (excluded, email))?;
        Ok(__cordl_ret)
    }
    pub fn UnionEmail_String_ISet1(
        &mut self,
        email1: *mut crate::System::String,
        email2: *mut crate::System::String,
        _cordl_union: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnionEmail", (email1, email2, _cordl_union))?;
        Ok(__cordl_ret)
    }
    pub fn IsUriConstrained_String0(
        &mut self,
        constraint: *mut crate::System::String,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsUriConstrained", (constraint, uri))?;
        Ok(__cordl_ret)
    }
    pub fn IsUriConstrained_ISet1(
        &mut self,
        constraints: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsUriConstrained", (constraints, uri))?;
        Ok(__cordl_ret)
    }
    pub fn CheckPermittedUri(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckPermittedUri", (permitted, uri))?;
        Ok(__cordl_ret)
    }
    pub fn IsEmailConstrained_String0(
        &mut self,
        constraint: *mut crate::System::String,
        email: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEmailConstrained", (constraint, email))?;
        Ok(__cordl_ret)
    }
    pub fn IsEmailConstrained_ISet1(
        &mut self,
        constraints: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        email: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEmailConstrained", (constraints, email))?;
        Ok(__cordl_ret)
    }
    pub fn SpecialEquals(
        &mut self,
        o1: *mut crate::System::Object,
        o2: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SpecialEquals", (o1, o2))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckPermittedEmail(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        email: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckPermittedEmail", (permitted, email))?;
        Ok(__cordl_ret)
    }
    pub fn IsIPConstrained_Il2CppArray0(
        &mut self,
        constraint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ip: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsIPConstrained", (constraint, ip))?;
        Ok(__cordl_ret)
    }
    pub fn IsIPConstrained_ISet1(
        &mut self,
        constraints: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        ip: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsIPConstrained", (constraints, ip))?;
        Ok(__cordl_ret)
    }
    pub fn CheckPermittedOtherName(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        name: *mut crate::Org::BouncyCastle::Asn1::X509::OtherName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckPermittedOtherName", (permitted, name))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectEmptyPermittedSubtree(
        &mut self,
        nameType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IntersectEmptyPermittedSubtree", (nameType))?;
        Ok(__cordl_ret)
    }
    pub fn CheckPermittedIP(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        ip: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckPermittedIP", (permitted, ip))?;
        Ok(__cordl_ret)
    }
    pub fn HashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("HashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn IntersectIP(
        &mut self,
        permitted: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        ips: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("IntersectIP", (permitted, ips))?;
        Ok(__cordl_ret)
    }
    pub fn StringifyIP(
        &mut self,
        ip: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("StringifyIP", (ip))?;
        Ok(__cordl_ret)
    }
    pub fn CheckPermittedDN(
        &mut self,
        dn: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckPermittedDN", (dn))?;
        Ok(__cordl_ret)
    }
    pub fn CheckExcludedDns(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        dns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckExcludedDns", (excluded, dns))?;
        Ok(__cordl_ret)
    }
    pub fn IsOtherNameConstrained_OtherName0(
        &mut self,
        constraint: *mut crate::Org::BouncyCastle::Asn1::X509::OtherName,
        otherName: *mut crate::Org::BouncyCastle::Asn1::X509::OtherName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsOtherNameConstrained", (constraint, otherName))?;
        Ok(__cordl_ret)
    }
    pub fn IsOtherNameConstrained_ISet1(
        &mut self,
        constraints: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        otherName: *mut crate::Org::BouncyCastle::Asn1::X509::OtherName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsOtherNameConstrained", (constraints, otherName))?;
        Ok(__cordl_ret)
    }
    pub fn UnionUri(
        &mut self,
        excluded: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("UnionUri", (excluded, uri))?;
        Ok(__cordl_ret)
    }
    pub fn checkExcluded(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("checkExcluded", (name))?;
        Ok(__cordl_ret)
    }
    pub fn MinMaxIPs(
        &mut self,
        ip1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        subnetmask1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ip2: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        subnetmask2: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("MinMaxIPs", (ip1, subnetmask1, ip2, subnetmask2))?;
        Ok(__cordl_ret)
    }
    pub fn CheckExcludedDN(
        &mut self,
        dn: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckExcludedDN", (dn))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixNameConstraintValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
