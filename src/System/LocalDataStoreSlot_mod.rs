#[cfg(feature = "System+LocalDataStoreSlot")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalDataStoreSlot {
    __cordl_parent: crate::System::Object,
    pub m_mgr: *mut crate::System::LocalDataStoreMgr,
    pub m_slot: i32,
    pub m_cookie: i64,
}
#[cfg(feature = "System+LocalDataStoreSlot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::LocalDataStoreSlot => "System"
    ."LocalDataStoreSlot"
);
#[cfg(feature = "System+LocalDataStoreSlot")]
impl std::ops::Deref for crate::System::LocalDataStoreSlot {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        mgr: *mut crate::System::LocalDataStoreMgr,
        slot: i32,
        cookie: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mgr, slot, cookie))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_Manager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::LocalDataStoreMgr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::LocalDataStoreMgr = __cordl_object
            .invoke("get_Manager", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Slot(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Slot", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Cookie(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Cookie", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        mgr: *mut crate::System::LocalDataStoreMgr,
        slot: i32,
        cookie: i64,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mgr, slot, cookie))?;
        Ok(__cordl_object)
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
