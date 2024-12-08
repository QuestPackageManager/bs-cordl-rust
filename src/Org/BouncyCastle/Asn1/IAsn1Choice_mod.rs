#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1Choice")]
#[repr(C)]
#[derive(Debug)]
pub struct IAsn1Choice {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1Choice")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::IAsn1Choice =>
    "Org.BouncyCastle.Asn1"."IAsn1Choice"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1Choice")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::IAsn1Choice {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1Choice")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::IAsn1Choice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1Choice")]
impl crate::Org::BouncyCastle::Asn1::IAsn1Choice {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1Choice")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::IAsn1Choice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
