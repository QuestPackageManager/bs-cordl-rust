#[cfg(feature = "CustomLevelsSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomLevelsSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub enabled: bool,
}
#[cfg(feature = "CustomLevelsSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CustomLevelsSettings => ""
    ."CustomLevelsSettings"
);
#[cfg(feature = "CustomLevelsSettings")]
impl std::ops::Deref for crate::GlobalNamespace::CustomLevelsSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CustomLevelsSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::CustomLevelsSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CustomLevelsSettings")]
impl crate::GlobalNamespace::CustomLevelsSettings {
    pub fn New(
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (enabled))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (enabled))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CustomLevelsSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CustomLevelsSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
