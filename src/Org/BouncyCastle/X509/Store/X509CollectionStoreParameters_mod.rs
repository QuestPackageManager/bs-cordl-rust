#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStoreParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CollectionStoreParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub collection: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStoreParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::X509::Store::X509CollectionStoreParameters =>
    "Org.BouncyCastle.X509.Store"."X509CollectionStoreParameters"
);
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStoreParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::X509::Store::X509CollectionStoreParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStoreParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::X509::Store::X509CollectionStoreParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStoreParameters")]
impl crate::Org::BouncyCastle::X509::Store::X509CollectionStoreParameters {
    pub fn GetCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("GetCollection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        collection: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collection))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        collection: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collection))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStoreParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::Store::X509CollectionStoreParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStoreParameters")]
impl AsRef<crate::Org::BouncyCastle::X509::Store::IX509StoreParameters>
for crate::Org::BouncyCastle::X509::Store::X509CollectionStoreParameters {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::X509::Store::IX509StoreParameters {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CollectionStoreParameters")]
impl AsMut<crate::Org::BouncyCastle::X509::Store::IX509StoreParameters>
for crate::Org::BouncyCastle::X509::Store::X509CollectionStoreParameters {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::X509::Store::IX509StoreParameters {
        unsafe { std::mem::transmute(self) }
    }
}
