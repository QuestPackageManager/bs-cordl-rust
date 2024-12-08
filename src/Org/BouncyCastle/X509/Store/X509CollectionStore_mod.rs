#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStore")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CollectionStore {
    __cordl_parent: crate::System::Object,
    pub _local: *mut crate::System::Collections::ICollection,
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::X509::Store::X509CollectionStore =>
    "Org.BouncyCastle.X509.Store"."X509CollectionStore"
);
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStore")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::Store::X509CollectionStore {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStore")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::Store::X509CollectionStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStore")]
impl crate::Org::BouncyCastle::X509::Store::X509CollectionStore {
    pub fn _ctor(
        &mut self,
        collection: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collection))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatches(
        &mut self,
        selector: *mut crate::Org::BouncyCastle::X509::Store::IX509Selector,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("GetMatches", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        collection: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collection))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::Store::X509CollectionStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
