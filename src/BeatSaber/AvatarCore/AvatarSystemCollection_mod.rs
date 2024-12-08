#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarSystemCollection {
    __cordl_parent: crate::System::Object,
    pub _availableAvatarSystems: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
    >,
    pub _availableUserSelectableAvatarSystems: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
    >,
    pub _supportedOptionalAvatarDataTypes: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        u32,
    >,
    pub _fallbackAvatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
    pub _availableAvatarSystemList: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::AvatarSystemCollection =>
    "BeatSaber.AvatarCore"."AvatarSystemCollection"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
impl crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    #[cfg(
        feature = "BeatSaber+AvatarCore+AvatarSystemCollection+_GetMultiplayerAvatarsData_d__16"
    )]
    pub type _GetMultiplayerAvatarsData_d__16 = crate::BeatSaber::AvatarCore::AvatarSystemCollection__GetMultiplayerAvatarsData_d__16;
    #[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection+__c")]
    pub type __c = crate::BeatSaber::AvatarCore::AvatarSystemCollection___c;
    #[cfg(
        feature = "BeatSaber+AvatarCore+AvatarSystemCollection+_CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync_d__18"
    )]
    pub type _CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync_d__18 = crate::BeatSaber::AvatarCore::AvatarSystemCollection__CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync_d__18;
    pub fn _ctor(
        &mut self,
        boundAvatarSystems: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (boundAvatarSystems))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectableAvatarSystems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        > = __cordl_object.invoke("get_selectableAvatarSystems", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasAvatarSystem(
        &mut self,
        avatarTypeIdentifierHash: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasAvatarSystem", (avatarTypeIdentifierHash))?;
        Ok(__cordl_ret)
    }
    pub fn get_anyAvatarCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("get_anyAvatarCreated", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_availableAvatarSystems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        > = __cordl_object.invoke("get_availableAvatarSystems", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_supportedOptionalAvatarDataType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyCollection_1<u32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            u32,
        > = __cordl_object.invoke("get_supportedOptionalAvatarDataType", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMultiplayerAvatarOptionalDataProvider(
        &mut self,
        selectedAvatarTypeIdentifier: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider = __cordl_object
            .invoke(
                "GetMultiplayerAvatarOptionalDataProvider",
                (selectedAvatarTypeIdentifier),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetAvatarSystem_IAvatarSystemMetadata0(
        &mut self,
        avatarSystemMetadata: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::IAvatarSystem = __cordl_object
            .invoke("GetAvatarSystem", (avatarSystemMetadata))?;
        Ok(__cordl_ret)
    }
    pub fn GetAvatarSystem_AvatarSystemIdentifier1(
        &mut self,
        avatarSystemIdentifier: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::IAvatarSystem = __cordl_object
            .invoke("GetAvatarSystem", (avatarSystemIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn GetAvatarSystem_String2(
        &mut self,
        avatarTypeIdentifier: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::IAvatarSystem = __cordl_object
            .invoke("GetAvatarSystem", (avatarTypeIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn GetAvatarSystem_u32_3(
        &mut self,
        avatarTypeIdentifierHash: u32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::IAvatarSystem = __cordl_object
            .invoke("GetAvatarSystem", (avatarTypeIdentifierHash))?;
        Ok(__cordl_ret)
    }
    pub fn GetMultiplayerAvatarsData(
        &mut self,
        selectedAvatarTypeIdentifier: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<MultiplayerAvatarsData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            MultiplayerAvatarsData,
        > = __cordl_object
            .invoke("GetMultiplayerAvatarsData", (selectedAvatarTypeIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        boundAvatarSystems: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (boundAvatarSystems))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
