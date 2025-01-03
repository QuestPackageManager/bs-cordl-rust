#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarSystemCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn AvatarSystemBySelectionSortOrder(
        system1: quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
        system2: quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AvatarSystemBySelectionSortOrder", (system1, system2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarSystem_AvatarSystemIdentifier1(
        &mut self,
        avatarSystemIdentifier: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystem,
        > = __cordl_object.invoke("GetAvatarSystem", (avatarSystemIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarSystem_IAvatarSystemMetadata0(
        &mut self,
        avatarSystemMetadata: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystem,
        > = __cordl_object.invoke("GetAvatarSystem", (avatarSystemMetadata))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarSystem_Il2CppString2(
        &mut self,
        avatarTypeIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystem,
        > = __cordl_object.invoke("GetAvatarSystem", (avatarTypeIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarSystem_u32_3(
        &mut self,
        avatarTypeIdentifierHash: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystem,
        > = __cordl_object.invoke("GetAvatarSystem", (avatarTypeIdentifierHash))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMultiplayerAvatarOptionalDataProvider(
        &mut self,
        selectedAvatarTypeIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
        > = __cordl_object
            .invoke(
                "GetMultiplayerAvatarOptionalDataProvider",
                (selectedAvatarTypeIdentifier),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMultiplayerAvatarsData(
        &mut self,
        selectedAvatarTypeIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::MultiplayerAvatarsData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::MultiplayerAvatarsData,
            >,
        > = __cordl_object
            .invoke("GetMultiplayerAvatarsData", (selectedAvatarTypeIdentifier))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        boundAvatarSystems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (boundAvatarSystems))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        boundAvatarSystems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (boundAvatarSystems))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_anyAvatarCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("get_anyAvatarCreated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_availableAvatarSystems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        > = __cordl_object.invoke("get_availableAvatarSystems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectableAvatarSystems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        > = __cordl_object.invoke("get_selectableAvatarSystems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_supportedOptionalAvatarDataType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<u32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<u32>,
        > = __cordl_object.invoke("get_supportedOptionalAvatarDataType", ())?;
        Ok(__cordl_ret.into())
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
