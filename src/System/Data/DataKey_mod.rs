#[cfg(feature = "System+Data+DataKey")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DataKey {
    pub _columns: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::DataColumn,
    >,
}
#[cfg(feature = "System+Data+DataKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataKey => "System.Data"."DataKey"
);
#[cfg(feature = "System+Data+DataKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Data::DataKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+DataKey")]
impl crate::System::Data::DataKey {
    pub fn CheckState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckState",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ColumnsEqual(
        &mut self,
        key: crate::System::Data::DataKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ColumnsEqual",
            (key),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ContainsColumn(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ContainsColumn",
            (column),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_DataKey1(
        &mut self,
        value: crate::System::Data::DataKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetColumnNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetColumnNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexDesc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
    > {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Data::IndexField,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetIndexDesc", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyValues(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetKeyValues", (record))?;
        Ok(__cordl_ret)
    }
    pub fn GetSortIndex_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Index> {
        let __cordl_ret: *mut crate::System::Data::Index = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSortIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetSortIndex_DataViewRowState1(
        &mut self,
        recordStates: crate::System::Data::DataViewRowState,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Index> {
        let __cordl_ret: *mut crate::System::Data::Index = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSortIndex",
            (recordStates),
        )?;
        Ok(__cordl_ret)
    }
    pub fn RecordsEqual(
        &mut self,
        record1: i32,
        record2: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RecordsEqual",
            (record1, record2),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        copyColumns: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (columns, copyColumns),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ColumnsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ColumnsReference",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_HasValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Table(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_ret: *mut crate::System::Data::DataTable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Table",
            (),
        )?;
        Ok(__cordl_ret)
    }
}