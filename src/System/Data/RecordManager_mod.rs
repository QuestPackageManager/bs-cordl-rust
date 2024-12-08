#[cfg(feature = "System+Data+RecordManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordManager {
    __cordl_parent: crate::System::Object,
    pub _table: *mut crate::System::Data::DataTable,
    pub _lastFreeRecord: i32,
    pub _minimumCapacity: i32,
    pub _recordCapacity: i32,
    pub _freeRecordList: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _rows: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::DataRow,
    >,
}
#[cfg(feature = "System+Data+RecordManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RecordManager => "System.Data"
    ."RecordManager"
);
#[cfg(feature = "System+Data+RecordManager")]
impl std::ops::Deref for crate::System::Data::RecordManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+RecordManager")]
impl std::ops::DerefMut for crate::System::Data::RecordManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+RecordManager")]
impl crate::System::Data::RecordManager {
    pub fn Clear(
        &mut self,
        clearAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", (clearAll))?;
        Ok(__cordl_ret)
    }
    pub fn CopyRecord(
        &mut self,
        src: *mut crate::System::Data::DataTable,
        record: i32,
        copy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CopyRecord", (src, record, copy))?;
        Ok(__cordl_ret)
    }
    pub fn FreeRecord(
        &mut self,
        record: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeRecord", (record))?;
        Ok(__cordl_ret)
    }
    pub fn GrowRecordCapacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GrowRecordCapacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImportRecord(
        &mut self,
        src: *mut crate::System::Data::DataTable,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ImportRecord", (src, record))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table))?;
        Ok(__cordl_object)
    }
    pub fn NewRecordBase(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NewRecordBase", ())?;
        Ok(__cordl_ret)
    }
    pub fn NormalizedMinimumCapacity(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("NormalizedMinimumCapacity", (capacity))?;
        Ok(__cordl_ret)
    }
    pub fn SetRowCache(
        &mut self,
        newRows: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataRow,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRowCache", (newRows))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("get_Item", (record))?;
        Ok(__cordl_ret)
    }
    pub fn get_LastFreeRecord(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LastFreeRecord", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MinimumCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MinimumCapacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RecordCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RecordCapacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Item(
        &mut self,
        record: i32,
        value: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (record, value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MinimumCapacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MinimumCapacity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RecordCapacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RecordCapacity", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+RecordManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::RecordManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
