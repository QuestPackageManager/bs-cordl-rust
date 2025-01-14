#[cfg(feature = "System+Data+RelatedView")]
#[repr(C)]
#[derive(Debug)]
pub struct RelatedView {
    __cordl_parent: crate::System::Data::DataView,
    pub _parentKey: crate::System::Nullable_1<crate::System::Data::DataKey>,
    pub _childKey: crate::System::Data::DataKey,
    pub _parentRowView: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowView>,
    pub _filterValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
}
#[cfg(feature = "System+Data+RelatedView")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::RelatedView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "RelatedView";
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
    pub fn AddNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRowView>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRowView>,
                0usize,
            >("AddNew")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddNew", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowView> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
                0usize,
            >("GetFilter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFilter", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetParentValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                0usize,
            >("GetParentValues")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetParentValues", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowVersion,
                ),
                bool,
                2usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Invoke", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (row, version)) };
        Ok(__cordl_ret.into())
    }
    pub fn New_DataRowView_DataKey_Il2CppArray1(
        parentRowView: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowView>,
        parentKey: crate::System::Data::DataKey,
        childKeyColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentRowView, parentKey, childKeyColumns))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_Il2CppArray0(
        columns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        >,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns, values))?;
        Ok(__cordl_object.into())
    }
    pub fn SetIndex(
        &mut self,
        newSort: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newRowStates: crate::System::Data::DataViewRowState,
        newRowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Data::DataViewRowState,
                    quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetIndex", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newSort, newRowStates, newRowFilter))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataRowView_DataKey_Il2CppArray1(
        &mut self,
        parentRowView: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowView>,
        parentKey: crate::System::Data::DataKey,
        childKeyColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRowView>,
                    crate::System::Data::DataKey,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parentRowView, parentKey, childKeyColumns))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_Il2CppArray0(
        &mut self,
        columns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        >,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (columns, values))
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Data+RelatedView")]
impl AsRef<crate::System::Data::IFilter> for crate::System::Data::RelatedView {
    fn as_ref(&self) -> &crate::System::Data::IFilter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+RelatedView")]
impl AsMut<crate::System::Data::IFilter> for crate::System::Data::RelatedView {
    fn as_mut(&mut self) -> &mut crate::System::Data::IFilter {
        unsafe { std::mem::transmute(self) }
    }
}
