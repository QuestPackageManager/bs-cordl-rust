#[cfg(feature = "System+Linq+Expressions+InvocationExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct InvocationExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _Type_k__BackingField: *mut crate::System::Type,
    pub _Expression_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::InvocationExpression
    => "System.Linq.Expressions"."InvocationExpression"
);
#[cfg(feature = "System+Linq+Expressions+InvocationExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::InvocationExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::InvocationExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression")]
impl crate::System::Linq::Expressions::InvocationExpression {
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
    pub fn GetArgument(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("GetArgument", (index))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        expression: *mut crate::System::Linq::Expressions::Expression,
        returnType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression, returnType))?;
        Ok(__cordl_object)
    }
    pub fn Rewrite(
        &mut self,
        lambda: *mut crate::System::Linq::Expressions::Expression,
        arguments: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::InvocationExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::InvocationExpression = __cordl_object
            .invoke("Rewrite", (lambda, arguments))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        expression: *mut crate::System::Linq::Expressions::Expression,
        returnType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression, returnType))?;
        Ok(__cordl_ret)
    }
    pub fn get_ArgumentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgumentCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Expression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_Expression", ())?;
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
#[cfg(feature = "System+Linq+Expressions+InvocationExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::InvocationExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}