#[cfg(feature = "System+Linq+Expressions+LogicalBinaryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct LogicalBinaryExpression {
    __cordl_parent: crate::System::Linq::Expressions::BinaryExpression,
    pub _NodeType_k__BackingField: crate::System::Linq::Expressions::ExpressionType,
}
#[cfg(feature = "System+Linq+Expressions+LogicalBinaryExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::LogicalBinaryExpression => "System.Linq.Expressions"
    ."LogicalBinaryExpression"
);
#[cfg(feature = "System+Linq+Expressions+LogicalBinaryExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::LogicalBinaryExpression {
    type Target = crate::System::Linq::Expressions::BinaryExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+LogicalBinaryExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::LogicalBinaryExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+LogicalBinaryExpression")]
impl crate::System::Linq::Expressions::LogicalBinaryExpression {
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
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nodeType, left, right))?;
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
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nodeType, left, right))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+LogicalBinaryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::LogicalBinaryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
