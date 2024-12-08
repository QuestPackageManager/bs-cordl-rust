#[cfg(feature = "System+Linq+Expressions+FullConditionalExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct FullConditionalExpression {
    __cordl_parent: crate::System::Linq::Expressions::ConditionalExpression,
    pub _false: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::FullConditionalExpression => "System.Linq.Expressions"
    ."FullConditionalExpression"
);
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::FullConditionalExpression {
    type Target = crate::System::Linq::Expressions::ConditionalExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::FullConditionalExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpression")]
impl crate::System::Linq::Expressions::FullConditionalExpression {
    pub fn _ctor(
        &mut self,
        test: *mut crate::System::Linq::Expressions::Expression,
        ifTrue: *mut crate::System::Linq::Expressions::Expression,
        ifFalse: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (test, ifTrue, ifFalse))?;
        Ok(__cordl_ret)
    }
    pub fn GetFalse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("GetFalse", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        test: *mut crate::System::Linq::Expressions::Expression,
        ifTrue: *mut crate::System::Linq::Expressions::Expression,
        ifFalse: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (test, ifTrue, ifFalse))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::FullConditionalExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
