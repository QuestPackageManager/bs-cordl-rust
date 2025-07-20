#[cfg(feature = "System+Data+ExpressionNode")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
}
#[cfg(feature = "System+Data+ExpressionNode")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::ExpressionNode {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "ExpressionNode";
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
#[cfg(feature = "System+Data+ExpressionNode")]
impl std::ops::Deref for crate::System::Data::ExpressionNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ExpressionNode")]
impl std::ops::DerefMut for crate::System::Data::ExpressionNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ExpressionNode")]
impl crate::System::Data::ExpressionNode {
    pub fn Bind(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Bind")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "Bind", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table, list))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BindTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BindTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "BindTable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DependsOn(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                bool,
                1usize,
            >("DependsOn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "DependsOn", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (column))? };
        Ok(__cordl_ret.into())
    }
    pub fn Eval_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("Eval")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "Eval", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Eval_DataRow_DataRowVersion1(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowVersion,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("Eval")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "Eval", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (row, version))? };
        Ok(__cordl_ret.into())
    }
    pub fn Eval_Il2CppArray2(
        &mut self,
        recordNos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("Eval")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "Eval", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (recordNos))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasLocalAggregate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("HasLocalAggregate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "HasLocalAggregate", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HasRemoteAggregate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("HasRemoteAggregate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "HasRemoteAggregate", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsConstant")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsConstant", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsFloat(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsFloat", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsFloatSql(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsFloatSql")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsFloatSql", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsInteger(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsInteger", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsIntegerSql(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsIntegerSql")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsIntegerSql", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsNumeric(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsNumeric")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsNumeric", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsNumericSql(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsNumericSql")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsNumericSql", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSigned(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsSigned")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsSigned", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSignedSql(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsSignedSql")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsSignedSql", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsTableConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsTableConstant")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsTableConstant", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsUnsigned(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsUnsigned")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsUnsigned", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsUnsignedSql(
        _cordl_type: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Data::Common::StorageType),
                bool,
                1usize,
            >("IsUnsignedSql")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "IsUnsignedSql", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table))?;
        Ok(__cordl_object.into())
    }
    pub fn Optimize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
                0usize,
            >("Optimize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "Optimize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ExpressionNode,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_FormatProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                0usize,
            >("get_FormatProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "get_FormatProvider", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSqlColumn(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsSqlColumn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "get_IsSqlColumn", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_table(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::ExpressionNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                0usize,
            >("get_table")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::ExpressionNode as quest_hook::libil2cpp::Type >
                    ::class(), "get_table", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+ExpressionNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ExpressionNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
