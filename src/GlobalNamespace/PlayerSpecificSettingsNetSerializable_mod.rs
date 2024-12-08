#[cfg(feature = "PlayerSpecificSettingsNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSpecificSettingsNetSerializable {
    __cordl_parent: crate::System::Object,
    pub userId: *mut crate::System::String,
    pub userName: *mut crate::System::String,
    pub leftHanded: bool,
    pub automaticPlayerHeight: bool,
    pub playerHeight: f32,
    pub headPosToPlayerHeightOffset: f32,
    pub colorScheme: crate::GlobalNamespace::ColorSchemeNetSerializable,
}
#[cfg(feature = "PlayerSpecificSettingsNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSpecificSettingsNetSerializable => ""
    ."PlayerSpecificSettingsNetSerializable"
);
#[cfg(feature = "PlayerSpecificSettingsNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettingsNetSerializable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettingsNetSerializable")]
impl crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_String__cordl_bool__cordl_bool_f32_f32_Color_Color_Color_Color_Color_Color_Color1(
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        leftHanded: bool,
        automaticPlayerHeight: bool,
        playerHeight: f32,
        headPosToPlayerHeightOffset: f32,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    userId,
                    userName,
                    leftHanded,
                    automaticPlayerHeight,
                    playerHeight,
                    headPosToPlayerHeightOffset,
                    saberAColor,
                    saberBColor,
                    obstaclesColor,
                    environmentColor0,
                    environmentColor1,
                    environmentColor0Boost,
                    environmentColor1Boost,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String__cordl_bool__cordl_bool_f32_f32_Color_Color_Color_Color_Color_Color_Color1(
        &mut self,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        leftHanded: bool,
        automaticPlayerHeight: bool,
        playerHeight: f32,
        headPosToPlayerHeightOffset: f32,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    userId,
                    userName,
                    leftHanded,
                    automaticPlayerHeight,
                    playerHeight,
                    headPosToPlayerHeightOffset,
                    saberAColor,
                    saberBColor,
                    obstaclesColor,
                    environmentColor0,
                    environmentColor1,
                    environmentColor0Boost,
                    environmentColor1Boost,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayerSpecificSettingsNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
