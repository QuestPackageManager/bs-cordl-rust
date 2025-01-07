#[cfg(feature = "System+LocalDataStoreSlot")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalDataStoreSlot {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_mgr: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreMgr>,
    pub m_slot: i32,
    pub m_cookie: i64,
}
#[cfg(feature = "System+LocalDataStoreSlot")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::LocalDataStoreSlot {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "LocalDataStoreSlot";
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
#[cfg(feature = "System+LocalDataStoreSlot")]
impl std::ops::Deref for crate::System::LocalDataStoreSlot {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalDataStoreSlot")]
impl std::ops::DerefMut for crate::System::LocalDataStoreSlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalDataStoreSlot")]
impl crate::System::LocalDataStoreSlot {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        mgr: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreMgr>,
        slot: i32,
        cookie: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mgr, slot, cookie))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        mgr: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreMgr>,
        slot: i32,
        cookie: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mgr, slot, cookie))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Cookie(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Cookie", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Manager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreMgr>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreMgr> = __cordl_object
            .invoke("get_Manager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Slot(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Slot", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+LocalDataStoreSlot")]
impl quest_hook::libil2cpp::ObjectType for crate::System::LocalDataStoreSlot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
