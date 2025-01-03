#[cfg(feature = "PracticeSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PracticeSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _startSongTime: f32,
    pub _songSpeedMul: f32,
    pub _startInAdvanceAndClearNotes: bool,
}
#[cfg(feature = "PracticeSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PracticeSettings => ""
    ."PracticeSettings"
);
#[cfg(feature = "PracticeSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PracticeSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PracticeSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::PracticeSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PracticeSettings")]
impl crate::GlobalNamespace::PracticeSettings {
    pub const kDelayBeforeStart: f32 = 1f32;
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_PracticeSettings1(
        practiceSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (practiceSettings))?;
        Ok(__cordl_object.into())
    }
    pub fn New_f32_f32_2(
        startSongTime: f32,
        songSpeedMul: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (startSongTime, songSpeedMul))?;
        Ok(__cordl_object.into())
    }
    pub fn ResetToDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetToDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PracticeSettings1(
        &mut self,
        practiceSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (practiceSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_2(
        &mut self,
        startSongTime: f32,
        songSpeedMul: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (startSongTime, songSpeedMul))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPracticeSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultPracticeSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songSpeedMul(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songSpeedMul", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startInAdvanceAndClearNotes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_startInAdvanceAndClearNotes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startSongTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startSongTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_songSpeedMul(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_songSpeedMul", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startInAdvanceAndClearNotes(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startInAdvanceAndClearNotes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSongTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startSongTime", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PracticeSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PracticeSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
