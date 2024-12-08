#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
#[repr(C)]
#[derive(Debug)]
pub struct DisplayText {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub contentType: i32,
    pub contents: *mut crate::Org::BouncyCastle::Asn1::IAsn1String,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::DisplayText =>
    "Org.BouncyCastle.Asn1.X509"."DisplayText"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    pub const ContentTypeBmpString: i32 = 1i32;
    pub const ContentTypeIA5String: i32 = 0i32;
    pub const ContentTypeUtf8String: i32 = 2i32;
    pub const ContentTypeVisibleString: i32 = 3i32;
    pub const DisplayTextMaximumSize: i32 = 200i32;
    pub fn GetString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IAsn1String2(
        contents: *mut crate::Org::BouncyCastle::Asn1::IAsn1String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contents))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (text))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_String0(
        _cordl_type: i32,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, text))?;
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
    pub fn _ctor_IAsn1String2(
        &mut self,
        contents: *mut crate::Org::BouncyCastle::Asn1::IAsn1String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contents))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (text))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_String0(
        &mut self,
        _cordl_type: i32,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, text))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
