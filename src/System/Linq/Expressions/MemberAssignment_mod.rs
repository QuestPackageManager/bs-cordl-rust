#[cfg(feature = "System+Linq+Expressions+MemberAssignment")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberAssignment {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::MemberBinding,
    >,
    pub _expression: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >,
}
#[cfg(feature = "System+Linq+Expressions+MemberAssignment")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::MemberAssignment =>
    "System.Linq.Expressions"."MemberAssignment"
);
#[cfg(feature = "System+Linq+Expressions+MemberAssignment")]
impl std::ops::Deref for crate::System::Linq::Expressions::MemberAssignment {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::MemberBinding,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberAssignment")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MemberAssignment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberAssignment")]
impl crate::System::Linq::Expressions::MemberAssignment {
    pub fn get_Expression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("get_Expression", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberAssignment")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MemberAssignment {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
