#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+AdditionalInformationSyntax")]
#[repr(C)]
#[derive(Debug)]
pub struct AdditionalInformationSyntax {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub information: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+AdditionalInformationSyntax")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IsisMtt::X509::AdditionalInformationSyntax =>
    "Org.BouncyCastle.Asn1.IsisMtt.X509"."AdditionalInformationSyntax"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+AdditionalInformationSyntax")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::AdditionalInformationSyntax {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+AdditionalInformationSyntax")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::AdditionalInformationSyntax {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+AdditionalInformationSyntax")]
impl crate::Org::BouncyCastle::Asn1::IsisMtt::X509::AdditionalInformationSyntax {
    pub fn New_DirectoryString0(
        information: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (information))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        information: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (information))?;
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
    pub fn _ctor_DirectoryString0(
        &mut self,
        information: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (information))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        information: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (information))?;
        Ok(__cordl_ret)
    }
    pub fn get_Information(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString = __cordl_object
            .invoke("get_Information", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+AdditionalInformationSyntax")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::AdditionalInformationSyntax {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}