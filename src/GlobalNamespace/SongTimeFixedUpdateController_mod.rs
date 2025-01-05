#[cfg(feature = "SongTimeFixedUpdateController")]
#[repr(C)]
#[derive(Debug)]
pub struct SongTimeFixedUpdateController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioTimeSource: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAudioTimeSource,
    >,
    pub songControllerFixedTimeDidUpdateEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<f32>,
    >,
    pub songControllerTimeDidUpdateEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub _accumulator: f32,
    pub _interpolationFactor: f32,
}
#[cfg(feature = "SongTimeFixedUpdateController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongTimeFixedUpdateController
    => ""."SongTimeFixedUpdateController"
);
#[cfg(feature = "SongTimeFixedUpdateController")]
impl std::ops::Deref for crate::GlobalNamespace::SongTimeFixedUpdateController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongTimeFixedUpdateController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongTimeFixedUpdateController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongTimeFixedUpdateController")]
impl crate::GlobalNamespace::SongTimeFixedUpdateController {
    pub const kFixedDeltaTime: f32 = 0.016666668f32;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
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
    pub fn add_songControllerFixedTimeDidUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_songControllerFixedTimeDidUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_songControllerTimeDidUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_songControllerTimeDidUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedDeltaTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fixedDeltaTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interpolationFactor(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_interpolationFactor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_songControllerFixedTimeDidUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_songControllerFixedTimeDidUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_songControllerTimeDidUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_songControllerTimeDidUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongTimeFixedUpdateController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SongTimeFixedUpdateController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
