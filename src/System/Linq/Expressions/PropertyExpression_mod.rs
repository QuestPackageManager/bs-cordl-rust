#[cfg(feature = "System+Linq+Expressions+PropertyExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyExpression {
    __cordl_parent: crate::System::Linq::Expressions::MemberExpression,
    pub _property: *mut crate::System::Reflection::PropertyInfo,
}
#[cfg(feature = "System+Linq+Expressions+PropertyExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::PropertyExpression =>
    "System.Linq.Expressions"."PropertyExpression"
);
#[cfg(feature = "System+Linq+Expressions+PropertyExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::PropertyExpression {
    type Target = crate::System::Linq::Expressions::MemberExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+PropertyExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::PropertyExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+PropertyExpression")]
impl crate::System::Linq::Expressions::PropertyExpression {
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
        expression: *mut crate::System::Linq::Expressions::Expression,
        member: *mut crate::System::Reflection::PropertyInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression, member))?;
        Ok(__cordl_ret)
    }
    pub fn GetMember(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MemberInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MemberInfo = __cordl_object
            .invoke("GetMember", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        expression: *mut crate::System::Linq::Expressions::Expression,
        member: *mut crate::System::Reflection::PropertyInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression, member))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+PropertyExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::PropertyExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
