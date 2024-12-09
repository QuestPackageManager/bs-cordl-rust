#[cfg(feature = "BeatSaber+GameSettings+LegacyModelConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacyModelConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fileStorage: *mut crate::GlobalNamespace::IFileStorage,
    pub _legacyModel: *mut crate::BeatSaber::GameSettings::LegacySettingsModel,
}
#[cfg(feature = "BeatSaber+GameSettings+LegacyModelConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::LegacyModelConverter =>
    "BeatSaber.GameSettings"."LegacyModelConverter"
);
#[cfg(feature = "BeatSaber+GameSettings+LegacyModelConverter")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::LegacyModelConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+LegacyModelConverter")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::LegacyModelConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+LegacyModelConverter")]
impl crate::BeatSaber::GameSettings::LegacyModelConverter {
    pub const kFileName: &'static str = "settings.cfg";
    #[cfg(
        feature = "BeatSaber+GameSettings+LegacyModelConverter+_AttemptConversionAsync_d__5"
    )]
    pub type _AttemptConversionAsync_d__5 = crate::BeatSaber::GameSettings::LegacyModelConverter__AttemptConversionAsync_d__5;
    #[cfg(
        feature = "BeatSaber+GameSettings+LegacyModelConverter+_RequiresUpdateAsync_d__4"
    )]
    pub type _RequiresUpdateAsync_d__4 = crate::BeatSaber::GameSettings::LegacyModelConverter__RequiresUpdateAsync_d__4;
    pub fn AttemptConversionAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::System::ValueTuple_2<
                *mut crate::BeatSaber::GameSettings::MainSettings,
                *mut crate::BeatSaber::GameSettings::GraphicSettings,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::ValueTuple_2<
                *mut crate::BeatSaber::GameSettings::MainSettings,
                *mut crate::BeatSaber::GameSettings::GraphicSettings,
            >,
        > = __cordl_object.invoke("AttemptConversionAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        fileStorage: *mut crate::GlobalNamespace::IFileStorage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileStorage))?;
        Ok(__cordl_object)
    }
    pub fn RequiresUpdateAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("RequiresUpdateAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        fileStorage: *mut crate::GlobalNamespace::IFileStorage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileStorage))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+GameSettings+LegacyModelConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::LegacyModelConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
