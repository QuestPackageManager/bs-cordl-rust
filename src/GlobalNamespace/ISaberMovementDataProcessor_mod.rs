#[cfg(feature = "ISaberMovementDataProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct ISaberMovementDataProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISaberMovementDataProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ISaberMovementDataProcessor =>
    ""."ISaberMovementDataProcessor"
);
#[cfg(feature = "ISaberMovementDataProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::ISaberMovementDataProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberMovementDataProcessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::ISaberMovementDataProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberMovementDataProcessor")]
impl crate::GlobalNamespace::ISaberMovementDataProcessor {
    pub fn ProcessNewData(
        &mut self,
        newData: crate::GlobalNamespace::BladeMovementDataElement,
        prevData: crate::GlobalNamespace::BladeMovementDataElement,
        prevDataAreValid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewData", (newData, prevData, prevDataAreValid))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISaberMovementDataProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ISaberMovementDataProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
