#[cfg(feature = "System+Linq+Expressions+MemberBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberBinding {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _BindingType_k__BackingField: crate::System::Linq::Expressions::MemberBindingType,
    pub _Member_k__BackingField: *mut crate::System::Reflection::MemberInfo,
}
#[cfg(feature = "System+Linq+Expressions+MemberBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::MemberBinding =>
    "System.Linq.Expressions"."MemberBinding"
);
#[cfg(feature = "System+Linq+Expressions+MemberBinding")]
impl std::ops::Deref for crate::System::Linq::Expressions::MemberBinding {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberBinding")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MemberBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberBinding")]
impl crate::System::Linq::Expressions::MemberBinding {
    pub fn get_BindingType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::MemberBindingType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::MemberBindingType = __cordl_object
            .invoke("get_BindingType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Member(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MemberInfo,
        > = __cordl_object.invoke("get_Member", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MemberBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
