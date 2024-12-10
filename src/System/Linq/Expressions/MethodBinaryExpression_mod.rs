#[cfg(feature = "System+Linq+Expressions+MethodBinaryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct MethodBinaryExpression {
    __cordl_parent: crate::System::Linq::Expressions::SimpleBinaryExpression,
    pub _method: *mut crate::System::Reflection::MethodInfo,
}
#[cfg(feature = "System+Linq+Expressions+MethodBinaryExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::MethodBinaryExpression => "System.Linq.Expressions"
    ."MethodBinaryExpression"
);
#[cfg(feature = "System+Linq+Expressions+MethodBinaryExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::MethodBinaryExpression {
    type Target = crate::System::Linq::Expressions::SimpleBinaryExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodBinaryExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MethodBinaryExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodBinaryExpression")]
impl crate::System::Linq::Expressions::MethodBinaryExpression {
    pub fn GetMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("GetMethod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nodeType, left, right, _cordl_type, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nodeType, left, right, _cordl_type, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodBinaryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MethodBinaryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
