#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpressionN")]
#[repr(C)]
#[derive(Debug)]
pub struct InstanceMethodCallExpressionN {
    __cordl_parent: crate::System::Linq::Expressions::InstanceMethodCallExpression,
    pub _arguments: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::System::Linq::Expressions::Expression,
    >,
}
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpressionN")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::InstanceMethodCallExpressionN =>
    "System.Linq.Expressions"."InstanceMethodCallExpressionN"
);
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpressionN")]
impl std::ops::Deref
for crate::System::Linq::Expressions::InstanceMethodCallExpressionN {
    type Target = crate::System::Linq::Expressions::InstanceMethodCallExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpressionN")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::InstanceMethodCallExpressionN {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpressionN")]
impl crate::System::Linq::Expressions::InstanceMethodCallExpressionN {
    pub fn _ctor(
        &mut self,
        method: *mut crate::System::Reflection::MethodInfo,
        instance: *mut crate::System::Linq::Expressions::Expression,
        args: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, instance, args))?;
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
    pub fn get_ArgumentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgumentCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn Rewrite(
        &mut self,
        instance: *mut crate::System::Linq::Expressions::Expression,
        args: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::MethodCallExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::MethodCallExpression = __cordl_object
            .invoke("Rewrite", (instance, args))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        method: *mut crate::System::Reflection::MethodInfo,
        instance: *mut crate::System::Linq::Expressions::Expression,
        args: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, instance, args))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpressionN")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::InstanceMethodCallExpressionN {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
