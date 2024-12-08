#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberListBinding {
    __cordl_parent: crate::System::Linq::Expressions::MemberBinding,
    pub _Initializers_k__BackingField: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
        *mut crate::System::Linq::Expressions::ElementInit,
    >,
}
#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::MemberListBinding =>
    "System.Linq.Expressions"."MemberListBinding"
);
#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
impl std::ops::Deref for crate::System::Linq::Expressions::MemberListBinding {
    type Target = crate::System::Linq::Expressions::MemberBinding;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MemberListBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
impl crate::System::Linq::Expressions::MemberListBinding {
    pub fn get_Initializers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::ElementInit,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::ElementInit,
        > = __cordl_object.invoke("get_Initializers", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MemberListBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
