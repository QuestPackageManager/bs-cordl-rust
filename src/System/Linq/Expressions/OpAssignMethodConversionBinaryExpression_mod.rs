#[cfg(feature = "System+Linq+Expressions+OpAssignMethodConversionBinaryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct OpAssignMethodConversionBinaryExpression {
    __cordl_parent: crate::System::Linq::Expressions::MethodBinaryExpression,
    pub _conversion: *mut crate::System::Linq::Expressions::LambdaExpression,
}
#[cfg(feature = "System+Linq+Expressions+OpAssignMethodConversionBinaryExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::OpAssignMethodConversionBinaryExpression =>
    "System.Linq.Expressions"."OpAssignMethodConversionBinaryExpression"
);
#[cfg(feature = "System+Linq+Expressions+OpAssignMethodConversionBinaryExpression")]
impl std::ops::Deref
for crate::System::Linq::Expressions::OpAssignMethodConversionBinaryExpression {
    type Target = crate::System::Linq::Expressions::MethodBinaryExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+OpAssignMethodConversionBinaryExpression")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::OpAssignMethodConversionBinaryExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+OpAssignMethodConversionBinaryExpression")]
impl crate::System::Linq::Expressions::OpAssignMethodConversionBinaryExpression {
    pub fn GetConversion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::LambdaExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::LambdaExpression = __cordl_object
            .invoke("GetConversion", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
        _cordl_type: *mut crate::System::Type,
        method: *mut crate::System::Reflection::MethodInfo,
        conversion: *mut crate::System::Linq::Expressions::LambdaExpression,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (nodeType, left, right, _cordl_type, method, conversion),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
        _cordl_type: *mut crate::System::Type,
        method: *mut crate::System::Reflection::MethodInfo,
        conversion: *mut crate::System::Linq::Expressions::LambdaExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nodeType, left, right, _cordl_type, method, conversion))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+OpAssignMethodConversionBinaryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::OpAssignMethodConversionBinaryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
