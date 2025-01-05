#[cfg(feature = "GameplayModifiersConfiguration")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiersConfiguration {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GameplayModifiersConfiguration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifiersConfiguration
    => ""."GameplayModifiersConfiguration"
);
#[cfg(feature = "GameplayModifiersConfiguration")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifiersConfiguration {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersConfiguration")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifiersConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersConfiguration")]
impl crate::GlobalNamespace::GameplayModifiersConfiguration {
    #[cfg(feature = "GameplayModifiersConfiguration+CutAngleTolerance")]
    pub type CutAngleTolerance = crate::GlobalNamespace::GameplayModifiersConfiguration_CutAngleTolerance;
    #[cfg(feature = "GameplayModifiersConfiguration+NoteUniformScale")]
    pub type NoteUniformScale = crate::GlobalNamespace::GameplayModifiersConfiguration_NoteUniformScale;
    #[cfg(feature = "GameplayModifiersConfiguration+SongSpeed")]
    pub type SongSpeed = crate::GlobalNamespace::GameplayModifiersConfiguration_SongSpeed;
}
#[cfg(feature = "GameplayModifiersConfiguration")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifiersConfiguration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+CutAngleTolerance")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiersConfiguration_CutAngleTolerance {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GameplayModifiersConfiguration+CutAngleTolerance")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayModifiersConfiguration_CutAngleTolerance => ""
    ."GameplayModifiersConfiguration/CutAngleTolerance"
);
#[cfg(feature = "GameplayModifiersConfiguration+CutAngleTolerance")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayModifiersConfiguration_CutAngleTolerance {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+CutAngleTolerance")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayModifiersConfiguration_CutAngleTolerance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+CutAngleTolerance")]
impl crate::GlobalNamespace::GameplayModifiersConfiguration_CutAngleTolerance {
    pub const kDefault: f32 = 60f32;
    pub const kStrict: f32 = 40f32;
}
#[cfg(feature = "GameplayModifiersConfiguration+CutAngleTolerance")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifiersConfiguration_CutAngleTolerance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+NoteUniformScale")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiersConfiguration_NoteUniformScale {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GameplayModifiersConfiguration+NoteUniformScale")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayModifiersConfiguration_NoteUniformScale => ""
    ."GameplayModifiersConfiguration/NoteUniformScale"
);
#[cfg(feature = "GameplayModifiersConfiguration+NoteUniformScale")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayModifiersConfiguration_NoteUniformScale {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+NoteUniformScale")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayModifiersConfiguration_NoteUniformScale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+NoteUniformScale")]
impl crate::GlobalNamespace::GameplayModifiersConfiguration_NoteUniformScale {
    pub const kDefault: f32 = 1f32;
    pub const kSmall: f32 = 0.5f32;
}
#[cfg(feature = "GameplayModifiersConfiguration+NoteUniformScale")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifiersConfiguration_NoteUniformScale {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+SongSpeed")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiersConfiguration_SongSpeed {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GameplayModifiersConfiguration+SongSpeed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayModifiersConfiguration_SongSpeed => ""
    ."GameplayModifiersConfiguration/SongSpeed"
);
#[cfg(feature = "GameplayModifiersConfiguration+SongSpeed")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayModifiersConfiguration_SongSpeed {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+SongSpeed")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayModifiersConfiguration_SongSpeed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersConfiguration+SongSpeed")]
impl crate::GlobalNamespace::GameplayModifiersConfiguration_SongSpeed {
    pub const kFaster: f32 = 1.2f32;
    pub const kNormal: f32 = 1f32;
    pub const kSlower: f32 = 0.85f32;
    pub const kSuperFast: f32 = 1.5f32;
}
#[cfg(feature = "GameplayModifiersConfiguration+SongSpeed")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifiersConfiguration_SongSpeed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
