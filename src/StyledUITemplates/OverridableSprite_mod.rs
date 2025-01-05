#[cfg(feature = "StyledUITemplates+OverridableSprite")]
#[repr(C)]
#[derive(Debug)]
pub struct OverridableSprite {
    __cordl_parent: crate::StyledUITemplates::OverridableData_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    >,
}
#[cfg(feature = "StyledUITemplates+OverridableSprite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::StyledUITemplates::OverridableSprite =>
    "StyledUITemplates"."OverridableSprite"
);
#[cfg(feature = "StyledUITemplates+OverridableSprite")]
impl std::ops::Deref for crate::StyledUITemplates::OverridableSprite {
    type Target = crate::StyledUITemplates::OverridableData_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StyledUITemplates+OverridableSprite")]
impl std::ops::DerefMut for crate::StyledUITemplates::OverridableSprite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StyledUITemplates+OverridableSprite")]
impl crate::StyledUITemplates::OverridableSprite {
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
#[cfg(feature = "StyledUITemplates+OverridableSprite")]
impl quest_hook::libil2cpp::ObjectType for crate::StyledUITemplates::OverridableSprite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
