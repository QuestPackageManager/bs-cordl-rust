#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct InstanceMethodCallExpression {
    __cordl_parent: crate::System::Linq::Expressions::MethodCallExpression,
    pub _instance: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::InstanceMethodCallExpression =>
    "System.Linq.Expressions"."InstanceMethodCallExpression"
);
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::InstanceMethodCallExpression {
    type Target = crate::System::Linq::Expressions::MethodCallExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpression")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::InstanceMethodCallExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpression")]
impl crate::System::Linq::Expressions::InstanceMethodCallExpression {
    pub fn GetInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("GetInstance", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        method: *mut crate::System::Reflection::MethodInfo,
        instance: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, instance))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        method: *mut crate::System::Reflection::MethodInfo,
        instance: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, instance))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+InstanceMethodCallExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::InstanceMethodCallExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
