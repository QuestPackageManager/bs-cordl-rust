#[cfg(feature = "System+Data+BinaryNode")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryNode {
    __cordl_parent: crate::System::Data::ExpressionNode,
    pub _op: i32,
    pub _left: *mut crate::System::Data::ExpressionNode,
    pub _right: *mut crate::System::Data::ExpressionNode,
}
#[cfg(feature = "System+Data+BinaryNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::BinaryNode => "System.Data"
    ."BinaryNode"
);
#[cfg(feature = "System+Data+BinaryNode")]
impl std::ops::Deref for crate::System::Data::BinaryNode {
    type Target = crate::System::Data::ExpressionNode;
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
    pub fn BinaryCompare_CompareInfo1(
        &mut self,
        vLeft: *mut crate::System::Object,
        vRight: *mut crate::System::Object,
        resultType: crate::System::Data::Common::StorageType,
        op: i32,
        comparer: *mut crate::System::Globalization::CompareInfo,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BinaryCompare", (vLeft, vRight, resultType, op, comparer))?;
        Ok(__cordl_ret)
    }
    pub fn BinaryCompare_Object_Object_StorageType_i32_0(
        &mut self,
        vLeft: *mut crate::System::Object,
        vRight: *mut crate::System::Object,
        resultType: crate::System::Data::Common::StorageType,
        op: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BinaryCompare", (vLeft, vRight, resultType, op))?;
        Ok(__cordl_ret)
    }
    pub fn Bind(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        list: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Bind", (table, list))?;
        Ok(__cordl_ret)
    }
    pub fn DependsOn(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DependsOn", (column))?;
        Ok(__cordl_ret)
    }
    pub fn EvalBinaryOp(
        &mut self,
        op: i32,
        left: *mut crate::System::Data::ExpressionNode,
        right: *mut crate::System::Data::ExpressionNode,
        row: *mut crate::System::Data::DataRow,
        version: crate::System::Data::DataRowVersion,
        recordNos: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("EvalBinaryOp", (op, left, right, row, version, recordNos))?;
        Ok(__cordl_ret)
    }
    pub fn Eval_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object.invoke("Eval", ())?;
        Ok(__cordl_ret)
    }
    pub fn Eval_DataRow_DataRowVersion1(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Eval", (row, version))?;
        Ok(__cordl_ret)
    }
    pub fn Eval_Il2CppArray2(
        &mut self,
        recordNos: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Eval", (recordNos))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn HasLocalAggregate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLocalAggregate", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasRemoteAggregate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasRemoteAggregate", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsConstant", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn IsTableConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTableConstant", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        table: *mut crate::System::Data::DataTable,
        op: i32,
        left: *mut crate::System::Data::ExpressionNode,
        right: *mut crate::System::Data::ExpressionNode,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, op, left, right))?;
        Ok(__cordl_object)
    }
    pub fn Optimize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::ExpressionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::ExpressionNode = __cordl_object
            .invoke("Optimize", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetTypeMismatchError(
        &mut self,
        op: i32,
        left: *mut crate::System::Type,
        right: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTypeMismatchError", (op, left, right))?;
        Ok(__cordl_ret)
    }
    pub fn SqlResultType(
        &mut self,
        typeCode: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("SqlResultType", (typeCode))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        op: i32,
        left: *mut crate::System::Data::ExpressionNode,
        right: *mut crate::System::Data::ExpressionNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, op, left, right))?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryNode_DataTypePrecedence {
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
