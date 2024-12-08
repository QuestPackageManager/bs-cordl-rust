#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DistributionPoint")]
#[repr(C)]
#[derive(Debug)]
pub struct DistributionPoint {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub distributionPoint: *mut crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
    pub reasons: *mut crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
    pub cRLIssuer: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DistributionPoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::DistributionPoint
    => "Org.BouncyCastle.Asn1.X509"."DistributionPoint"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DistributionPoint")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::DistributionPoint {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DistributionPoint")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::DistributionPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DistributionPoint")]
impl crate::Org::BouncyCastle::Asn1::X509::DistributionPoint {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DistributionPointName_ReasonFlags_GeneralNames1(
        distributionPointName: *mut crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
        reasons: *mut crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
        crlIssuer: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (distributionPointName, reasons, crlIssuer))?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
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
    pub fn _ctor_Asn1Sequence0(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DistributionPointName_ReasonFlags_GeneralNames1(
        &mut self,
        distributionPointName: *mut crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
        reasons: *mut crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
        crlIssuer: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (distributionPointName, reasons, crlIssuer))?;
        Ok(__cordl_ret)
    }
    pub fn appendObject(
        &mut self,
        buf: *mut crate::System::Text::StringBuilder,
        sep: *mut crate::System::String,
        name: *mut crate::System::String,
        val: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("appendObject", (buf, sep, name, val))?;
        Ok(__cordl_ret)
    }
    pub fn get_CrlIssuer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames = __cordl_object
            .invoke("get_CrlIssuer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DistributionPointName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::DistributionPointName = __cordl_object
            .invoke("get_DistributionPointName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Reasons(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::ReasonFlags = __cordl_object
            .invoke("get_Reasons", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DistributionPoint")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::DistributionPoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
