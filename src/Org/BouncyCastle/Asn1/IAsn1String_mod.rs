#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1String")]
#[repr(C)]
#[derive(Debug)]
pub struct IAsn1String {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1String")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::IAsn1String =>
    "Org.BouncyCastle.Asn1"."IAsn1String"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1String")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::IAsn1String {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1String")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::IAsn1String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1String")]
impl crate::Org::BouncyCastle::Asn1::IAsn1String {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1String")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::IAsn1String {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
