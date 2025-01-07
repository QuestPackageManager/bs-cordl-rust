#[cfg(feature = "System+LocalDataStore")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalDataStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_DataTable: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreElement>,
        >,
    >,
    pub m_Manager: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreMgr>,
}
#[cfg(feature = "System+LocalDataStore")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::LocalDataStore {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "LocalDataStore";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+LocalDataStore")]
impl std::ops::Deref for crate::System::LocalDataStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalDataStore")]
impl std::ops::DerefMut for crate::System::LocalDataStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalDataStore")]
impl crate::System::LocalDataStore {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeData(
        &mut self,
        slot: i32,
        cookie: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeData", (slot, cookie))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetData(
        &mut self,
        slot: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetData", (slot))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        mgr: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreMgr>,
        InitialCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mgr, InitialCapacity))?;
        Ok(__cordl_object.into())
    }
    pub fn PopulateElement(
        &mut self,
        slot: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::LocalDataStoreElement,
        > = __cordl_object.invoke("PopulateElement", (slot))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        slot: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (slot, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        mgr: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreMgr>,
        InitialCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mgr, InitialCapacity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+LocalDataStore")]
impl quest_hook::libil2cpp::ObjectType for crate::System::LocalDataStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
