#[cfg(feature = "System+Linq+Expressions+LoopExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct LoopExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _Body_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
    pub _BreakLabel_k__BackingField: *mut crate::System::Linq::Expressions::LabelTarget,
    pub _ContinueLabel_k__BackingField: *mut crate::System::Linq::Expressions::LabelTarget,
}
#[cfg(feature = "System+Linq+Expressions+LoopExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::LoopExpression =>
    "System.Linq.Expressions"."LoopExpression"
);
#[cfg(feature = "System+Linq+Expressions+LoopExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::LoopExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+LoopExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::LoopExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+LoopExpression")]
impl crate::System::Linq::Expressions::LoopExpression {
    pub fn get_BreakLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::LabelTarget,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::LabelTarget = __cordl_object
            .invoke("get_BreakLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        breakLabel: *mut crate::System::Linq::Expressions::LabelTarget,
        continueLabel: *mut crate::System::Linq::Expressions::LabelTarget,
        body: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::LoopExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::LoopExpression = __cordl_object
            .invoke("Update", (breakLabel, continueLabel, body))?;
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
    pub fn get_ContinueLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::LabelTarget,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::LabelTarget = __cordl_object
            .invoke("get_ContinueLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        body: *mut crate::System::Linq::Expressions::Expression,
        _cordl_break: *mut crate::System::Linq::Expressions::LabelTarget,
        _cordl_continue: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (body, _cordl_break, _cordl_continue))?;
        Ok(__cordl_ret)
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_Body", ())?;
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
    pub fn New(
        body: *mut crate::System::Linq::Expressions::Expression,
        _cordl_break: *mut crate::System::Linq::Expressions::LabelTarget,
        _cordl_continue: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (body, _cordl_break, _cordl_continue))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+LoopExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::LoopExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
