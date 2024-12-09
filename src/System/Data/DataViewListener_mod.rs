#[cfg(feature = "System+Data+DataViewListener")]
#[repr(C)]
#[derive(Debug)]
pub struct DataViewListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dvWeak: *mut crate::System::WeakReference,
    pub _table: *mut crate::System::Data::DataTable,
    pub _index: *mut crate::System::Data::Index,
    pub _objectID: i32,
}
#[cfg(feature = "System+Data+DataViewListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataViewListener => "System.Data"
    ."DataViewListener"
);
#[cfg(feature = "System+Data+DataViewListener")]
impl std::ops::Deref for crate::System::Data::DataViewListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewListener")]
impl std::ops::DerefMut for crate::System::Data::DataViewListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewListener")]
impl crate::System::Data::DataViewListener {
    pub fn ChildRelationCollectionChanged(
        &mut self,
        sender: *mut quest_hook::libil2cpp::Il2CppObject,
        e: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChildRelationCollectionChanged", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn CleanUp(
        &mut self,
        updateListeners: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUp", (updateListeners))?;
        Ok(__cordl_ret)
    }
    pub fn ColumnCollectionChanged(
        &mut self,
        sender: *mut quest_hook::libil2cpp::Il2CppObject,
        e: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColumnCollectionChanged", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn IndexListChanged(
        &mut self,
        e: *mut crate::System::ComponentModel::ListChangedEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IndexListChanged", (e))?;
        Ok(__cordl_ret)
    }
    pub fn MaintainDataView(
        &mut self,
        changedType: crate::System::ComponentModel::ListChangedType,
        row: *mut crate::System::Data::DataRow,
        trackAddRemove: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MaintainDataView", (changedType, row, trackAddRemove))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        dv: *mut crate::System::Data::DataView,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dv))?;
        Ok(__cordl_object)
    }
    pub fn ParentRelationCollectionChanged(
        &mut self,
        sender: *mut quest_hook::libil2cpp::Il2CppObject,
        e: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParentRelationCollectionChanged", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterListChangedEvent(
        &mut self,
        index: *mut crate::System::Data::Index,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterListChangedEvent", (index))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterListener(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterListener", (table))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterMetaDataEvents(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterMetaDataEvents", (table))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterListChangedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterListChangedEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterMetaDataEvents_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterMetaDataEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterMetaDataEvents__cordl_bool1(
        &mut self,
        updateListeners: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterMetaDataEvents", (updateListeners))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        dv: *mut crate::System::Data::DataView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dv))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+DataViewListener")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataViewListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
