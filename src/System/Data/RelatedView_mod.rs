#[cfg(feature = "System+Data+RelatedView")]
#[repr(C)]
#[derive(Debug)]
pub struct RelatedView {
    __cordl_parent: crate::System::Data::DataView,
    pub _parentKey: crate::System::Nullable_1<crate::System::Data::DataKey>,
    pub _childKey: crate::System::Data::DataKey,
    pub _parentRowView: *mut crate::System::Data::DataRowView,
    pub _filterValues: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "System+Data+RelatedView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RelatedView => "System.Data"
    ."RelatedView"
);
#[cfg(feature = "System+Data+RelatedView")]
impl std::ops::Deref for crate::System::Data::RelatedView {
    type Target = crate::System::Data::DataView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+RelatedView")]
impl std::ops::DerefMut for crate::System::Data::RelatedView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+RelatedView")]
impl crate::System::Data::RelatedView {
    pub fn Invoke(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (row, version))?;
        Ok(__cordl_ret)
    }
    pub fn GetFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::IFilter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::IFilter = __cordl_object
            .invoke("GetFilter", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRowView> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowView = __cordl_object
            .invoke("AddNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetParentValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetParentValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_Il2CppArray0(
        &mut self,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        values: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (columns, values))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataRowView_DataKey_Il2CppArray1(
        &mut self,
        parentRowView: *mut crate::System::Data::DataRowView,
        parentKey: crate::System::Data::DataKey,
        childKeyColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parentRowView, parentKey, childKeyColumns))?;
        Ok(__cordl_ret)
    }
    pub fn SetIndex(
        &mut self,
        newSort: *mut crate::System::String,
        newRowStates: crate::System::Data::DataViewRowState,
        newRowFilter: *mut crate::System::Data::IFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIndex", (newSort, newRowStates, newRowFilter))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray_Il2CppArray0(
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        values: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns, values))?;
        Ok(__cordl_object)
    }
    pub fn New_DataRowView_DataKey_Il2CppArray1(
        parentRowView: *mut crate::System::Data::DataRowView,
        parentKey: crate::System::Data::DataKey,
        childKeyColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentRowView, parentKey, childKeyColumns))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+RelatedView")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::RelatedView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
