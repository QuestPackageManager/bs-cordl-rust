#[cfg(feature = "Org+BouncyCastle+Asn1+OidTokenizer")]
#[repr(C)]
#[derive(Debug)]
pub struct OidTokenizer {
    __cordl_parent: crate::System::Object,
    pub oid: *mut crate::System::String,
    pub index: i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+OidTokenizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::OidTokenizer =>
    "Org.BouncyCastle.Asn1"."OidTokenizer"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+OidTokenizer")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::OidTokenizer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+OidTokenizer")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::OidTokenizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+OidTokenizer")]
impl crate::Org::BouncyCastle::Asn1::OidTokenizer {
    pub fn New(
        oid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid))?;
        Ok(__cordl_object)
    }
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
    pub fn _ctor(
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
    pub fn get_HasMoreTokens(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasMoreTokens", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+OidTokenizer")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::OidTokenizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
