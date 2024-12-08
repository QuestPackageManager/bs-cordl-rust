#[cfg(feature = "System+Linq+Expressions+UnaryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct UnaryExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _Type_k__BackingField: *mut crate::System::Type,
    pub _NodeType_k__BackingField: crate::System::Linq::Expressions::ExpressionType,
    pub _Operand_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
    pub _Method_k__BackingField: *mut crate::System::Reflection::MethodInfo,
}
#[cfg(feature = "System+Linq+Expressions+UnaryExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::UnaryExpression =>
    "System.Linq.Expressions"."UnaryExpression"
);
#[cfg(feature = "System+Linq+Expressions+UnaryExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::UnaryExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+UnaryExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::UnaryExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+UnaryExpression")]
impl crate::System::Linq::Expressions::UnaryExpression {
    pub fn Accept(
        &mut self,
        visitor: *mut crate::System::Linq::Expressions::ExpressionVisitor,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("Accept", (visitor))?;
        Ok(__cordl_ret)
    }
    pub fn FunctionalOp(
        &mut self,
        operand: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::UnaryExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::UnaryExpression = __cordl_object
            .invoke("FunctionalOp", (operand))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        expression: *mut crate::System::Linq::Expressions::Expression,
        _cordl_type: *mut crate::System::Type,
        method: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nodeType, expression, _cordl_type, method))?;
        Ok(__cordl_object)
    }
    pub fn Reduce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("Reduce", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReduceIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("ReduceIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReduceMember(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("ReduceMember", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReduceVariable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("ReduceVariable", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        operand: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::UnaryExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::UnaryExpression = __cordl_object
            .invoke("Update", (operand))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        expression: *mut crate::System::Linq::Expressions::Expression,
        _cordl_type: *mut crate::System::Type,
        method: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nodeType, expression, _cordl_type, method))?;
        Ok(__cordl_ret)
    }
    pub fn get_CanReduce(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanReduce", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsLifted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsLifted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsLiftedToNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsLiftedToNull", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPrefix(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPrefix", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("get_Method", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::ExpressionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::ExpressionType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Operand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_Operand", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+UnaryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::UnaryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
