#[cfg(feature = "StyledUITemplates+OverridableBool")]
#[repr(C)]
#[derive(Debug)]
pub struct OverridableBool {
    __cordl_parent: crate::StyledUITemplates::OverridableData_1<bool>,
}
#[cfg(feature = "StyledUITemplates+OverridableBool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::StyledUITemplates::OverridableBool =>
    "StyledUITemplates"."OverridableBool"
);
#[cfg(feature = "StyledUITemplates+OverridableBool")]
impl std::ops::Deref for crate::StyledUITemplates::OverridableBool {
    type Target = crate::StyledUITemplates::OverridableData_1<bool>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StyledUITemplates+OverridableBool")]
impl std::ops::DerefMut for crate::StyledUITemplates::OverridableBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StyledUITemplates+OverridableBool")]
impl crate::StyledUITemplates::OverridableBool {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StyledUITemplates+OverridableBool")]
impl quest_hook::libil2cpp::ObjectType for crate::StyledUITemplates::OverridableBool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
