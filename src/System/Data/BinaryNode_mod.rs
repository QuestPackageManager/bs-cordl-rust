#[cfg(feature = "System+Data+BinaryNode")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryNode {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    pub _op: i32,
    pub _left: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    pub _right: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
}
#[cfg(feature = "System+Data+BinaryNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::BinaryNode => "System.Data"
    ."BinaryNode"
);
#[cfg(feature = "System+Data+BinaryNode")]
impl std::ops::Deref for crate::System::Data::BinaryNode {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+BinaryNode")]
impl std::ops::DerefMut for crate::System::Data::BinaryNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+BinaryNode")]
impl crate::System::Data::BinaryNode {
    #[cfg(feature = "System+Data+BinaryNode+DataTypePrecedence")]
    pub type DataTypePrecedence = crate::System::Data::BinaryNode_DataTypePrecedence;
    pub fn BinaryCompare_Gc1(
        &mut self,
        vLeft: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vRight: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        resultType: crate::System::Data::Common::StorageType,
        op: i32,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BinaryCompare", (vLeft, vRight, resultType, op, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinaryCompare_Gc_Gc_StorageType_i32_0(
        &mut self,
        vLeft: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vRight: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        resultType: crate::System::Data::Common::StorageType,
        op: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BinaryCompare", (vLeft, vRight, resultType, op))?;
        Ok(__cordl_ret.into())
    }
    pub fn Bind(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        list: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
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
    pub fn EvalBinaryOp(
        &mut self,
        op: i32,
        left: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
        right: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
        recordNos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("EvalBinaryOp", (op, left, right, row, version, recordNos))?;
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
    pub fn Eval_Gc2(
        &mut self,
        recordNos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Eval", (recordNos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Eval_Gc_DataRowVersion1(
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
    pub fn Eval_Gc_Gc_DataRowVersion_Gc3(
        expr: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
        recordNos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Eval", (expr, row, version, recordNos))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrecedence(
        &mut self,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Data::BinaryNode_DataTypePrecedence,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::BinaryNode_DataTypePrecedence = __cordl_object
            .invoke("GetPrecedence", (storageType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrecedenceType(
        code: crate::System::Data::BinaryNode_DataTypePrecedence,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Common::StorageType> {
        let __cordl_ret: crate::System::Data::Common::StorageType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrecedenceType", (code))?;
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
    pub fn IsMixed(
        &mut self,
        left: crate::System::Data::Common::StorageType,
        right: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMixed", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMixedSql(
        &mut self,
        left: crate::System::Data::Common::StorageType,
        right: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMixedSql", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTableConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTableConstant", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        op: i32,
        left: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
        right: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, op, left, right))?;
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
    pub fn ResultSqlType(
        &mut self,
        left: crate::System::Data::Common::StorageType,
        right: crate::System::Data::Common::StorageType,
        lc: bool,
        rc: bool,
        op: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Common::StorageType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Common::StorageType = __cordl_object
            .invoke("ResultSqlType", (left, right, lc, rc, op))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResultType(
        &mut self,
        left: crate::System::Data::Common::StorageType,
        right: crate::System::Data::Common::StorageType,
        lc: bool,
        rc: bool,
        op: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Common::StorageType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Common::StorageType = __cordl_object
            .invoke("ResultType", (left, right, lc, rc, op))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTypeMismatchError(
        &mut self,
        op: i32,
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTypeMismatchError", (op, left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn SqlResultType(
        &mut self,
        typeCode: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("SqlResultType", (typeCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        op: i32,
        left: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
        right: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, op, left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+BinaryNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::BinaryNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+BinaryNode+DataTypePrecedence")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BinaryNode_DataTypePrecedence {
    #[default]
    Boolean = -2i32,
    Byte = 3i32,
    Char = -8i32,
    DateTime = 23i32,
    DateTimeOffset = 24i32,
    Decimal = 14i32,
    Double = 18i32,
    Error = 0i32,
    Int16 = 4i32,
    Int32 = 7i32,
    Int64 = 10i32,
    SByte = 1i32,
    Single = 16i32,
    SqlBinary = -10i32,
    SqlBoolean = -1i32,
    SqlByte = 2i32,
    SqlBytes = -9i32,
    SqlChars = -7i32,
    SqlDateTime = 25i32,
    SqlDecimal = 15i32,
    SqlDouble = 19i32,
    SqlGuid = -3i32,
    SqlInt16 = 5i32,
    SqlInt32 = 8i32,
    SqlInt64 = 11i32,
    SqlMoney = 13i32,
    SqlSingle = 17i32,
    SqlString = -4i32,
    SqlXml = -6i32,
    String = -5i32,
    TimeSpan = 20i32,
    UInt16 = 6i32,
    UInt32 = 9i32,
    UInt64 = 12i32,
}
#[cfg(feature = "System+Data+BinaryNode+DataTypePrecedence")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::BinaryNode_DataTypePrecedence =>
    "System.Data"."BinaryNode/DataTypePrecedence"
);
