#[cfg(feature = "System+Data+AggregateNode")]
#[repr(C)]
#[derive(Debug)]
pub struct AggregateNode {
    __cordl_parent: crate::System::Data::ExpressionNode,
    pub _type: crate::System::Data::AggregateType,
    pub _aggregate: crate::System::Data::Aggregate,
    pub _local: bool,
    pub _relationName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _columnName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _childTable: *mut crate::System::Data::DataTable,
    pub _column: *mut crate::System::Data::DataColumn,
    pub _relation: *mut crate::System::Data::DataRelation,
}
#[cfg(feature = "System+Data+AggregateNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::AggregateNode => "System.Data"
    ."AggregateNode"
);
#[cfg(feature = "System+Data+AggregateNode")]
impl std::ops::Deref for crate::System::Data::AggregateNode {
    type Target = crate::System::Data::ExpressionNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+AggregateNode")]
impl std::ops::DerefMut for crate::System::Data::AggregateNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+AggregateNode")]
impl crate::System::Data::AggregateNode {
    pub fn Bind_DataRelation1(
        relation: quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Data::DataColumn,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Bind", (relation, list))?;
        Ok(__cordl_ret.into())
    }
    pub fn Bind_DataTable0(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Data::DataColumn,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Bind", (table, list))?;
        Ok(__cordl_ret.into())
    }
    pub fn DependsOn(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DependsOn", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn Eval_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Eval", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Eval_DataRow_DataRowVersion1(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Eval", (row, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn Eval_Il2CppArray2(
        &mut self,
        records: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Eval", (records))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLocalAggregate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLocalAggregate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasRemoteAggregate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasRemoteAggregate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsConstant", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTableConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTableConstant", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DataTable_FunctionId_Il2CppString0(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        aggregateType: crate::System::Data::FunctionId,
        columnName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, aggregateType, columnName))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_Il2CppString1(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        aggregateType: crate::System::Data::FunctionId,
        columnName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        local: bool,
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (table, aggregateType, columnName, local, relationName),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Optimize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ExpressionNode,
        > = __cordl_object.invoke("Optimize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataTable_FunctionId_Il2CppString0(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        aggregateType: crate::System::Data::FunctionId,
        columnName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, aggregateType, columnName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_Il2CppString1(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        aggregateType: crate::System::Data::FunctionId,
        columnName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        local: bool,
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, aggregateType, columnName, local, relationName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+AggregateNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::AggregateNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
