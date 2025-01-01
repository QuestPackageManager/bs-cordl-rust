#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerProfileSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub alternativeHandling: bool,
    pub leftController: crate::BeatSaber::GameSettings::Controller,
    pub rightController: crate::BeatSaber::GameSettings::Controller,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::GameSettings::ControllerProfileSaveData => "BeatSaber.GameSettings"
    ."ControllerProfileSaveData"
);
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
impl crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    pub fn New(
        alternativeHandling: bool,
        leftController: crate::BeatSaber::GameSettings::Controller,
        rightController: crate::BeatSaber::GameSettings::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (alternativeHandling, leftController, rightController),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        alternativeHandling: bool,
        leftController: crate::BeatSaber::GameSettings::Controller,
        rightController: crate::BeatSaber::GameSettings::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (alternativeHandling, leftController, rightController))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
