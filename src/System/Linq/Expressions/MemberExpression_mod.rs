#[cfg(feature = "System+Linq+Expressions+MemberExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _Expression_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Linq+Expressions+MemberExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::MemberExpression =>
    "System.Linq.Expressions"."MemberExpression"
);
#[cfg(feature = "System+Linq+Expressions+MemberExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::MemberExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MemberExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberExpression")]
impl crate::System::Linq::Expressions::MemberExpression {
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
    pub fn GetMember(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MemberInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MemberInfo = __cordl_object
            .invoke("GetMember", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        expression: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression))?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
        expression: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::MemberExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::MemberExpression = __cordl_object
            .invoke("Update", (expression))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        expression: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression))?;
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
    pub fn get_Member(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MemberInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MemberInfo = __cordl_object
            .invoke("get_Member", ())?;
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
}
#[cfg(feature = "System+Linq+Expressions+MemberExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MemberExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}