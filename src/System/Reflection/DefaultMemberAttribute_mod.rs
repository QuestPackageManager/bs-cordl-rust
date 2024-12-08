#[cfg(feature = "System+Reflection+DefaultMemberAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultMemberAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _MemberName_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Reflection+DefaultMemberAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::DefaultMemberAttribute =>
    "System.Reflection"."DefaultMemberAttribute"
);
#[cfg(feature = "System+Reflection+DefaultMemberAttribute")]
impl std::ops::Deref for crate::System::Reflection::DefaultMemberAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+DefaultMemberAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::DefaultMemberAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+DefaultMemberAttribute")]
impl crate::System::Reflection::DefaultMemberAttribute {
    pub fn _ctor(
        &mut self,
        memberName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (memberName))?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_MemberName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        memberName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (memberName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Reflection+DefaultMemberAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::DefaultMemberAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
