#[cfg(feature = "System+LocalDataStoreMgr")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalDataStoreMgr {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_SlotInfoTable: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<bool>,
    >,
    pub m_FirstAvailableSlot: i32,
    pub m_ManagedLocalDataStores: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::LocalDataStore>,
        >,
    >,
    pub m_KeyToSlotMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
        >,
    >,
    pub m_CookieGenerator: i64,
}
#[cfg(feature = "System+LocalDataStoreMgr")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::LocalDataStoreMgr => "System"
    ."LocalDataStoreMgr"
);
#[cfg(feature = "System+LocalDataStoreMgr")]
impl std::ops::Deref for crate::System::LocalDataStoreMgr {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalDataStoreMgr")]
impl std::ops::DerefMut for crate::System::LocalDataStoreMgr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalDataStoreMgr")]
impl crate::System::LocalDataStoreMgr {
    pub const InitialSlotTableSize: i32 = 64i32;
    pub const LargeSlotTableSizeIncrease: i32 = 128i32;
    pub const SlotTableDoubleThreshold: i32 = 512i32;
    pub fn AllocateDataSlot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot> = __cordl_object
            .invoke("AllocateDataSlot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocateNamedDataSlot(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot> = __cordl_object
            .invoke("AllocateNamedDataSlot", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLocalDataStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreHolder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::LocalDataStoreHolder,
        > = __cordl_object.invoke("CreateLocalDataStore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteLocalDataStore(
        &mut self,
        store: quest_hook::libil2cpp::Gc<crate::System::LocalDataStore>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteLocalDataStore", (store))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeDataSlot(
        &mut self,
        slot: i32,
        cookie: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeDataSlot", (slot, cookie))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeNamedDataSlot(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeNamedDataSlot", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNamedDataSlot(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot> = __cordl_object
            .invoke("GetNamedDataSlot", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSlotTableLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSlotTableLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateSlot(
        &mut self,
        slot: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateSlot", (slot))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+LocalDataStoreMgr")]
impl quest_hook::libil2cpp::ObjectType for crate::System::LocalDataStoreMgr {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
