#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerProfilesSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: i32,
    pub profiles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::GameSettings::ControllerProfileSaveData,
        >,
    >,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::GameSettings::ControllerProfilesSaveData => "BeatSaber.GameSettings"
    ."ControllerProfilesSaveData"
);
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSaveData")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::ControllerProfilesSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSaveData")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::ControllerProfilesSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSaveData")]
impl crate::BeatSaber::GameSettings::ControllerProfilesSaveData {
    pub fn New(
        profiles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::BeatSaber::GameSettings::ControllerProfileSaveData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (profiles))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        profiles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::BeatSaber::GameSettings::ControllerProfileSaveData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (profiles))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::ControllerProfilesSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
