#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberInitExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _NewExpression_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::NewExpression,
    >,
    pub _Bindings_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberBinding>,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::MemberInitExpression {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "MemberInitExpression";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::MemberInitExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MemberInitExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
impl crate::System::Linq::Expressions::MemberInitExpression {
    pub fn get_Bindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::MemberBinding,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::MemberBinding,
                >,
            >,
        > = __cordl_object.invoke("get_Bindings", ())?;
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
#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MemberInitExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
