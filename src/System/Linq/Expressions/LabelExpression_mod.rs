#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct LabelExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _Target_k__BackingField: *mut crate::System::Linq::Expressions::LabelTarget,
    pub _DefaultValue_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::LabelExpression =>
    "System.Linq.Expressions"."LabelExpression"
);
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::LabelExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::LabelExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
impl crate::System::Linq::Expressions::LabelExpression {
    pub fn _ctor(
        &mut self,
        label: *mut crate::System::Linq::Expressions::LabelTarget,
        defaultValue: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn get_Target(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::LabelTarget,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::LabelTarget = __cordl_object
            .invoke("get_Target", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_DefaultValue", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn Update(
        &mut self,
        target: *mut crate::System::Linq::Expressions::LabelTarget,
        defaultValue: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::LabelExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::LabelExpression = __cordl_object
            .invoke("Update", (target, defaultValue))?;
        Ok(__cordl_ret)
    }
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
        label: *mut crate::System::Linq::Expressions::LabelTarget,
        defaultValue: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, defaultValue))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::LabelExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
