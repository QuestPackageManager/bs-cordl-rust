#[cfg(feature = "System+Linq+Expressions+AssignBinaryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct AssignBinaryExpression {
    __cordl_parent: crate::System::Linq::Expressions::BinaryExpression,
}
#[cfg(feature = "System+Linq+Expressions+AssignBinaryExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::AssignBinaryExpression => "System.Linq.Expressions"
    ."AssignBinaryExpression"
);
#[cfg(feature = "System+Linq+Expressions+AssignBinaryExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::AssignBinaryExpression {
    type Target = crate::System::Linq::Expressions::BinaryExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+AssignBinaryExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::AssignBinaryExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+AssignBinaryExpression")]
impl crate::System::Linq::Expressions::AssignBinaryExpression {
    pub fn New(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (left, right))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (left, right))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+AssignBinaryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::AssignBinaryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
