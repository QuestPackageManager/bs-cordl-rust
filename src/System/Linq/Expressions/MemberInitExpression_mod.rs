#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberInitExpression {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >,
    pub _NewExpression_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::NewExpression,
    >,
    pub _Bindings_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberBinding>,
    >,
}
#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::MemberInitExpression
    => "System.Linq.Expressions"."MemberInitExpression"
);
#[cfg(feature = "System+Linq+Expressions+MemberInitExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::MemberInitExpression {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >;
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
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberBinding>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberBinding>,
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
