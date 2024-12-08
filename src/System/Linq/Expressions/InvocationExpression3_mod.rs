#[cfg(feature = "System+Linq+Expressions+InvocationExpression3")]
#[repr(C)]
#[derive(Debug)]
pub struct InvocationExpression3 {
    __cordl_parent: crate::System::Linq::Expressions::InvocationExpression,
    pub _arg0: *mut crate::System::Object,
    pub _arg1: *mut crate::System::Linq::Expressions::Expression,
    pub _arg2: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::InvocationExpression3
    => "System.Linq.Expressions"."InvocationExpression3"
);
#[cfg(feature = "System+Linq+Expressions+InvocationExpression3")]
impl std::ops::Deref for crate::System::Linq::Expressions::InvocationExpression3 {
    type Target = crate::System::Linq::Expressions::InvocationExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression3")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::InvocationExpression3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression3")]
impl crate::System::Linq::Expressions::InvocationExpression3 {
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
        lambda: *mut crate::System::Linq::Expressions::Expression,
        returnType: *mut crate::System::Type,
        arg0: *mut crate::System::Linq::Expressions::Expression,
        arg1: *mut crate::System::Linq::Expressions::Expression,
        arg2: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lambda, returnType, arg0, arg1, arg2))?;
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
        lambda: *mut crate::System::Linq::Expressions::Expression,
        returnType: *mut crate::System::Type,
        arg0: *mut crate::System::Linq::Expressions::Expression,
        arg1: *mut crate::System::Linq::Expressions::Expression,
        arg2: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lambda, returnType, arg0, arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn get_ArgumentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgumentCount", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression3")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::InvocationExpression3 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
