#[cfg(feature = "System+Linq+Expressions+MethodCallExpression2")]
#[repr(C)]
#[derive(Debug)]
pub struct MethodCallExpression2 {
    __cordl_parent: crate::System::Linq::Expressions::MethodCallExpression,
    pub _arg0: *mut crate::System::Object,
    pub _arg1: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::MethodCallExpression2
    => "System.Linq.Expressions"."MethodCallExpression2"
);
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression2")]
impl std::ops::Deref for crate::System::Linq::Expressions::MethodCallExpression2 {
    type Target = crate::System::Linq::Expressions::MethodCallExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression2")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MethodCallExpression2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression2")]
impl crate::System::Linq::Expressions::MethodCallExpression2 {
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
        method: *mut crate::System::Reflection::MethodInfo,
        arg0: *mut crate::System::Linq::Expressions::Expression,
        arg1: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, arg0, arg1))?;
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        method: *mut crate::System::Reflection::MethodInfo,
        arg0: *mut crate::System::Linq::Expressions::Expression,
        arg1: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, arg0, arg1))?;
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
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression2")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MethodCallExpression2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
