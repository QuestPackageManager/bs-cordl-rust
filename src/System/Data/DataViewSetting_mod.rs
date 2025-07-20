#[cfg(feature = "System+Data+DataViewSetting")]
#[repr(C)]
#[derive(Debug)]
pub struct DataViewSetting {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dataViewManager: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataViewManager,
    >,
    pub _table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    pub _sort: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _rowFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _rowStateFilter: crate::System::Data::DataViewRowState,
    pub _applyDefaultSort: bool,
}
#[cfg(feature = "System+Data+DataViewSetting")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::DataViewSetting {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "DataViewSetting";
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
#[cfg(feature = "System+Data+DataViewSetting")]
impl std::ops::Deref for crate::System::Data::DataViewSetting {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewSetting")]
impl std::ops::DerefMut for crate::System::Data::DataViewSetting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewSetting")]
impl crate::System::Data::DataViewSetting {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetDataTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataViewSetting as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetDataTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataViewSetting as quest_hook::libil2cpp::Type
                    > ::class(), "SetDataTable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDataViewManager(
        &mut self,
        dataViewManager: quest_hook::libil2cpp::Gc<crate::System::Data::DataViewManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataViewSetting as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataViewManager>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetDataViewManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataViewSetting as quest_hook::libil2cpp::Type
                    > ::class(), "SetDataViewManager", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dataViewManager))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataViewSetting as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataViewSetting as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ApplyDefaultSort(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataViewSetting as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_ApplyDefaultSort")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataViewSetting as quest_hook::libil2cpp::Type
                    > ::class(), "get_ApplyDefaultSort", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_RowFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataViewSetting as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_RowFilter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataViewSetting as quest_hook::libil2cpp::Type
                    > ::class(), "get_RowFilter", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_RowStateFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataViewRowState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataViewSetting as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Data::DataViewRowState,
                0usize,
            >("get_RowStateFilter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataViewSetting as quest_hook::libil2cpp::Type
                    > ::class(), "get_RowStateFilter", 0usize
                )
            });
        let __cordl_ret: crate::System::Data::DataViewRowState = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Sort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataViewSetting as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Sort")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataViewSetting as quest_hook::libil2cpp::Type
                    > ::class(), "get_Sort", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataViewSetting")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataViewSetting {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
