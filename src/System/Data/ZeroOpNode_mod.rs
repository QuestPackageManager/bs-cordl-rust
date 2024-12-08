#[cfg(feature = "System+Data+ZeroOpNode")]
#[repr(C)]
#[derive(Debug)]
pub struct ZeroOpNode {
    __cordl_parent: crate::System::Data::ExpressionNode,
    pub _op: i32,
}
#[cfg(feature = "System+Data+ZeroOpNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ZeroOpNode => "System.Data"
    ."ZeroOpNode"
);
#[cfg(feature = "System+Data+ZeroOpNode")]
impl std::ops::Deref for crate::System::Data::ZeroOpNode {
    type Target = crate::System::Data::ExpressionNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ZeroOpNode")]
impl std::ops::DerefMut for crate::System::Data::ZeroOpNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ZeroOpNode")]
impl crate::System::Data::ZeroOpNode {
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
    pub fn IsTableConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTableConstant", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(op: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (op))?;
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
    pub fn _ctor(
        &mut self,
        op: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (op))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+ZeroOpNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ZeroOpNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
