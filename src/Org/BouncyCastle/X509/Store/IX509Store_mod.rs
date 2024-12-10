#[cfg(feature = "Org+BouncyCastle+X509+Store+IX509Store")]
#[repr(C)]
#[derive(Debug)]
pub struct IX509Store {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+IX509Store")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::Store::IX509Store =>
    "Org.BouncyCastle.X509.Store"."IX509Store"
);
#[cfg(feature = "Org+BouncyCastle+X509+Store+IX509Store")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::Store::IX509Store {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+IX509Store")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::Store::IX509Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+IX509Store")]
impl crate::Org::BouncyCastle::X509::Store::IX509Store {
    pub fn GetMatches(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Selector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("GetMatches", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+IX509Store")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::Store::IX509Store {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
