#[cfg(feature = "DeterminismConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct DeterminismConfig {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub forceSimplePhysics: bool,
}
#[cfg(feature = "DeterminismConfig")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DeterminismConfig => ""
    ."DeterminismConfig"
);
#[cfg(feature = "DeterminismConfig")]
impl std::ops::Deref for crate::GlobalNamespace::DeterminismConfig {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DeterminismConfig")]
impl std::ops::DerefMut for crate::GlobalNamespace::DeterminismConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DeterminismConfig")]
impl crate::GlobalNamespace::DeterminismConfig {
    pub fn New(
        deterministic: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (deterministic))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        deterministic: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (deterministic))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DeterminismConfig")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DeterminismConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}