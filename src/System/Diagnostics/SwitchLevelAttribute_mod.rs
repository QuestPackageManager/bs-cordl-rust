#[cfg(feature = "System+Diagnostics+SwitchLevelAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SwitchLevelAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+Diagnostics+SwitchLevelAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::SwitchLevelAttribute =>
    "System.Diagnostics"."SwitchLevelAttribute"
);
#[cfg(feature = "System+Diagnostics+SwitchLevelAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::SwitchLevelAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+SwitchLevelAttribute")]
impl std::ops::DerefMut for crate::System::Diagnostics::SwitchLevelAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+SwitchLevelAttribute")]
impl crate::System::Diagnostics::SwitchLevelAttribute {
    pub fn New(
        switchLevelType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (switchLevelType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        switchLevelType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (switchLevelType))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SwitchLevelType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SwitchLevelType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+SwitchLevelAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::SwitchLevelAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
