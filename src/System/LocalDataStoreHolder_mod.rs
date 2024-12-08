#[cfg(feature = "System+LocalDataStoreHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalDataStoreHolder {
    __cordl_parent: crate::System::Object,
    pub m_Store: *mut crate::System::LocalDataStore,
}
#[cfg(feature = "System+LocalDataStoreHolder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::LocalDataStoreHolder => "System"
    ."LocalDataStoreHolder"
);
#[cfg(feature = "System+LocalDataStoreHolder")]
impl std::ops::Deref for crate::System::LocalDataStoreHolder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalDataStoreHolder")]
impl std::ops::DerefMut for crate::System::LocalDataStoreHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalDataStoreHolder")]
impl crate::System::LocalDataStoreHolder {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        store: *mut crate::System::LocalDataStore,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (store))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        store: *mut crate::System::LocalDataStore,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (store))?;
        Ok(__cordl_ret)
    }
    pub fn get_Store(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::LocalDataStore> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::LocalDataStore = __cordl_object
            .invoke("get_Store", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+LocalDataStoreHolder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::LocalDataStoreHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}