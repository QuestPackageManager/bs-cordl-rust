#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProcurationSyntax")]
#[repr(C)]
#[derive(Debug)]
pub struct ProcurationSyntax {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub country: *mut crate::System::String,
    pub typeOfSubstitution: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    pub thirdPerson: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub certRef: *mut crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProcurationSyntax")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IsisMtt::X509::ProcurationSyntax =>
    "Org.BouncyCastle.Asn1.IsisMtt.X509"."ProcurationSyntax"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProcurationSyntax")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::ProcurationSyntax {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProcurationSyntax")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::ProcurationSyntax {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProcurationSyntax")]
impl crate::Org::BouncyCastle::Asn1::IsisMtt::X509::ProcurationSyntax {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_String_DirectoryString_GeneralName2(
        country: *mut crate::System::String,
        typeOfSubstitution: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        thirdPerson: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (country, typeOfSubstitution, thirdPerson))?;
        Ok(__cordl_object)
    }
    pub fn New_String_DirectoryString_IssuerSerial1(
        country: *mut crate::System::String,
        typeOfSubstitution: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        certRef: *mut crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (country, typeOfSubstitution, certRef))?;
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
    pub fn _ctor_String_DirectoryString_GeneralName2(
        &mut self,
        country: *mut crate::System::String,
        typeOfSubstitution: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        thirdPerson: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (country, typeOfSubstitution, thirdPerson))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_DirectoryString_IssuerSerial1(
        &mut self,
        country: *mut crate::System::String,
        typeOfSubstitution: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        certRef: *mut crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (country, typeOfSubstitution, certRef))?;
        Ok(__cordl_ret)
    }
    pub fn get_CertRef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::IssuerSerial = __cordl_object
            .invoke("get_CertRef", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Country(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Country", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ThirdPerson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName = __cordl_object
            .invoke("get_ThirdPerson", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeOfSubstitution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString = __cordl_object
            .invoke("get_TypeOfSubstitution", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProcurationSyntax")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::ProcurationSyntax {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
