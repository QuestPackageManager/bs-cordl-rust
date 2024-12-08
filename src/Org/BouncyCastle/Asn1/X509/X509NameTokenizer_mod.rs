#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509NameTokenizer")]
#[repr(C)]
#[derive(Debug)]
pub struct X509NameTokenizer {
    __cordl_parent: crate::System::Object,
    pub value: *mut crate::System::String,
    pub index: i32,
    pub separator: char,
    pub buffer: *mut crate::System::Text::StringBuilder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509NameTokenizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::X509NameTokenizer
    => "Org.BouncyCastle.Asn1.X509"."X509NameTokenizer"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509NameTokenizer")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::X509NameTokenizer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509NameTokenizer")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::X509NameTokenizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509NameTokenizer")]
impl crate::Org::BouncyCastle::Asn1::X509::X509NameTokenizer {
    pub fn NextToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("NextToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        oid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_char1(
        &mut self,
        oid: *mut crate::System::String,
        separator: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid, separator))?;
        Ok(__cordl_ret)
    }
    pub fn HasMoreTokens(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasMoreTokens", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        oid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_char1(
        oid: *mut crate::System::String,
        separator: char,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, separator))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509NameTokenizer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::X509NameTokenizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
