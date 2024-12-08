#[cfg(feature = "System+Linq+Expressions+CoalesceConversionBinaryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct CoalesceConversionBinaryExpression {
    __cordl_parent: crate::System::Linq::Expressions::BinaryExpression,
    pub _conversion: *mut crate::System::Linq::Expressions::LambdaExpression,
}
#[cfg(feature = "System+Linq+Expressions+CoalesceConversionBinaryExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::CoalesceConversionBinaryExpression =>
    "System.Linq.Expressions"."CoalesceConversionBinaryExpression"
);
#[cfg(feature = "System+Linq+Expressions+CoalesceConversionBinaryExpression")]
impl std::ops::Deref
for crate::System::Linq::Expressions::CoalesceConversionBinaryExpression {
    type Target = crate::System::Linq::Expressions::BinaryExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+CoalesceConversionBinaryExpression")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::CoalesceConversionBinaryExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+CoalesceConversionBinaryExpression")]
impl crate::System::Linq::Expressions::CoalesceConversionBinaryExpression {
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
    pub fn _ctor(
        &mut self,
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
        conversion: *mut crate::System::Linq::Expressions::LambdaExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (left, right, conversion))?;
        Ok(__cordl_ret)
    }
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
    pub fn New(
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
        conversion: *mut crate::System::Linq::Expressions::LambdaExpression,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (left, right, conversion))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+CoalesceConversionBinaryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::CoalesceConversionBinaryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
