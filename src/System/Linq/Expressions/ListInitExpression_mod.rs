#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct ListInitExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _NewExpression_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::NewExpression,
    >,
    pub _Initializers_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::ElementInit,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::ListInitExpression =>
    "System.Linq.Expressions"."ListInitExpression"
);
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::ListInitExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ListInitExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
impl crate::System::Linq::Expressions::ListInitExpression {
    pub fn get_Initializers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::ElementInit,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::ElementInit,
            >,
        > = __cordl_object.invoke("get_Initializers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NewExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::NewExpression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::NewExpression,
        > = __cordl_object.invoke("get_NewExpression", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ListInitExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
