#[cfg(feature = "Org+BouncyCastle+X509+Store+X509AttrCertStoreSelector")]
#[repr(C)]
#[derive(Debug)]
pub struct X509AttrCertStoreSelector {
    __cordl_parent: crate::System::Object,
    pub attributeCert: *mut crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
    pub attributeCertificateValid: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    pub holder: *mut crate::Org::BouncyCastle::X509::AttributeCertificateHolder,
    pub issuer: *mut crate::Org::BouncyCastle::X509::AttributeCertificateIssuer,
    pub serialNumber: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub targetNames: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub targetGroups: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509AttrCertStoreSelector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::X509::Store::X509AttrCertStoreSelector =>
    "Org.BouncyCastle.X509.Store"."X509AttrCertStoreSelector"
);
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509AttrCertStoreSelector")]
impl std::ops::Deref
for crate::Org::BouncyCastle::X509::Store::X509AttrCertStoreSelector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509AttrCertStoreSelector")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::X509::Store::X509AttrCertStoreSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509AttrCertStoreSelector")]
impl crate::Org::BouncyCastle::X509::Store::X509AttrCertStoreSelector {
    pub fn AddTargetGroup_GeneralName0(
        &mut self,
        group: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTargetGroup", (group))?;
        Ok(__cordl_ret)
    }
    pub fn AddTargetGroup_Il2CppArray1(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTargetGroup", (name))?;
        Ok(__cordl_ret)
    }
    pub fn AddTargetName_GeneralName0(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTargetName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn AddTargetName_Il2CppArray1(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTargetName", (name))?;
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
    pub fn ExtractGeneralNames(
        &mut self,
        names: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("ExtractGeneralNames", (names))?;
        Ok(__cordl_ret)
    }
    pub fn GetTargetGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetTargetGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTargetNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetTargetNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn Match(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Match", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_X509AttrCertStoreSelector1(
        o: *mut crate::Org::BouncyCastle::X509::Store::X509AttrCertStoreSelector,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object)
    }
    pub fn SetTargetGroups(
        &mut self,
        names: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetGroups", (names))?;
        Ok(__cordl_ret)
    }
    pub fn SetTargetNames(
        &mut self,
        names: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetNames", (names))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509AttrCertStoreSelector1(
        &mut self,
        o: *mut crate::Org::BouncyCastle::X509::Store::X509AttrCertStoreSelector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret)
    }
    pub fn get_AttribueCertificateValid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject = __cordl_object
            .invoke("get_AttribueCertificateValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeCert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::IX509AttributeCertificate = __cordl_object
            .invoke("get_AttributeCert", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeCertificateValid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject = __cordl_object
            .invoke("get_AttributeCertificateValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Holder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::AttributeCertificateHolder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::AttributeCertificateHolder = __cordl_object
            .invoke("get_Holder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Issuer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::AttributeCertificateIssuer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::AttributeCertificateIssuer = __cordl_object
            .invoke("get_Issuer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_SerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AttribueCertificateValid(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AttribueCertificateValid", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AttributeCert(
        &mut self,
        value: *mut crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AttributeCert", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AttributeCertificateValid(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AttributeCertificateValid", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Holder(
        &mut self,
        value: *mut crate::Org::BouncyCastle::X509::AttributeCertificateHolder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Holder", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Issuer(
        &mut self,
        value: *mut crate::Org::BouncyCastle::X509::AttributeCertificateIssuer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Issuer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SerialNumber(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SerialNumber", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509AttrCertStoreSelector")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::Store::X509AttrCertStoreSelector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}