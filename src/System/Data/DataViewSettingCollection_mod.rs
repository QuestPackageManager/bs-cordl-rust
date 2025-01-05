#[cfg(feature = "System+Data+DataViewSettingCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct DataViewSettingCollection {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _dataViewManager: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataViewManager,
    >,
    pub _list: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "System+Data+DataViewSettingCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataViewSettingCollection =>
    "System.Data"."DataViewSettingCollection"
);
#[cfg(feature = "System+Data+DataViewSettingCollection")]
impl std::ops::Deref for crate::System::Data::DataViewSettingCollection {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewSettingCollection")]
impl std::ops::DerefMut for crate::System::Data::DataViewSettingCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewSettingCollection")]
impl crate::System::Data::DataViewSettingCollection {
    pub fn Remove(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (table))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataViewSetting>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataViewSetting,
        > = __cordl_object.invoke("get_Item", (table))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        value: quest_hook::libil2cpp::Gc<crate::System::Data::DataViewSetting>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (table, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataViewSettingCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::DataViewSettingCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
