#[cfg(feature = "StyledUITemplates+OverridableFloat")]
#[repr(C)]
#[derive(Debug)]
pub struct OverridableFloat {
    __cordl_parent: crate::StyledUITemplates::OverridableData_1<f32>,
}
#[cfg(feature = "StyledUITemplates+OverridableFloat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::StyledUITemplates::OverridableFloat =>
    "StyledUITemplates"."OverridableFloat"
);
#[cfg(feature = "StyledUITemplates+OverridableFloat")]
impl std::ops::Deref for crate::StyledUITemplates::OverridableFloat {
    type Target = crate::StyledUITemplates::OverridableData_1<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StyledUITemplates+OverridableFloat")]
impl std::ops::DerefMut for crate::StyledUITemplates::OverridableFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StyledUITemplates+OverridableFloat")]
impl crate::StyledUITemplates::OverridableFloat {
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
#[cfg(feature = "StyledUITemplates+OverridableFloat")]
impl quest_hook::libil2cpp::ObjectType for crate::StyledUITemplates::OverridableFloat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
