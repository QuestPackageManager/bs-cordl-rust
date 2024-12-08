#[cfg(feature = "System+Linq+Expressions+SwitchExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct SwitchExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _SwitchValue_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
    pub _Cases_k__BackingField: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
        *mut crate::System::Linq::Expressions::SwitchCase,
    >,
    pub _DefaultBody_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
    pub _Comparison_k__BackingField: *mut crate::System::Reflection::MethodInfo,
}
#[cfg(feature = "System+Linq+Expressions+SwitchExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::SwitchExpression =>
    "System.Linq.Expressions"."SwitchExpression"
);
#[cfg(feature = "System+Linq+Expressions+SwitchExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::SwitchExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+SwitchExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::SwitchExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+SwitchExpression")]
impl crate::System::Linq::Expressions::SwitchExpression {
    pub fn get_SwitchValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_SwitchValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Cases(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::SwitchCase,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::SwitchCase,
        > = __cordl_object.invoke("get_Cases", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Comparison(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("get_Comparison", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultBody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_DefaultBody", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+SwitchExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::SwitchExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
