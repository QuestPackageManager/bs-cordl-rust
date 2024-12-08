#[cfg(feature = "System+Linq+Expressions+NewExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct NewExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _arguments: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::System::Linq::Expressions::Expression,
    >,
    pub _Constructor_k__BackingField: *mut crate::System::Reflection::ConstructorInfo,
}
#[cfg(feature = "System+Linq+Expressions+NewExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::NewExpression =>
    "System.Linq.Expressions"."NewExpression"
);
#[cfg(feature = "System+Linq+Expressions+NewExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::NewExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+NewExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::NewExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+NewExpression")]
impl crate::System::Linq::Expressions::NewExpression {
    pub fn GetArgument(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("GetArgument", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_Constructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::ConstructorInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::ConstructorInfo = __cordl_object
            .invoke("get_Constructor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+NewExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::NewExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
