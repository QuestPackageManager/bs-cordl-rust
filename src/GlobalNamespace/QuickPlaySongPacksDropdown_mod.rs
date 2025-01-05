#[cfg(feature = "QuickPlaySongPacksDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksDropdown {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _simpleTextDropdown: quest_hook::libil2cpp::Gc<crate::HMUI::SimpleTextDropdown>,
    pub _songPackMasksModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPackMasksModel,
    >,
    pub didSelectCellWithIdxEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<i32>,
    >,
    pub _initialized: bool,
    pub _quickPlaySongPacksOverride: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride,
    >,
    pub _data: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem,
            >,
        >,
    >,
}
#[cfg(feature = "QuickPlaySongPacksDropdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::QuickPlaySongPacksDropdown =>
    ""."QuickPlaySongPacksDropdown"
);
#[cfg(feature = "QuickPlaySongPacksDropdown")]
impl std::ops::Deref for crate::GlobalNamespace::QuickPlaySongPacksDropdown {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown")]
impl std::ops::DerefMut for crate::GlobalNamespace::QuickPlaySongPacksDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown")]
impl crate::GlobalNamespace::QuickPlaySongPacksDropdown {
    #[cfg(feature = "QuickPlaySongPacksDropdown+SongPackMaskItem")]
    pub type SongPackMaskItem = crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem;
    pub fn GetSelectedSerializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetSelectedSerializedName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSimpleTextDropdownDidSelectCellWithIdx(
        &mut self,
        dropdownWithTableView: quest_hook::libil2cpp::Gc<
            crate::HMUI::DropdownWithTableView,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSimpleTextDropdownDidSelectCellWithIdx",
                (dropdownWithTableView, idx),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LazyInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LazyInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectCellWithSerializedName(
        &mut self,
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectCellWithSerializedName", (serializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOverrideSongPacks(
        &mut self,
        quickPlaySongPacksOverride: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOverrideSongPacks", (quickPlaySongPacksOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _LazyInit_b__15_0(
        &mut self,
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem,
        > = __cordl_object.invoke("<LazyInit>b__15_0", (serializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _LazyInit_b__15_1(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem,
        > = __cordl_object.invoke("<LazyInit>b__15_1", (pack))?;
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
    pub fn add_didSelectCellWithIdxEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectCellWithIdxEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectCellWithIdxEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectCellWithIdxEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySongPacksDropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown+SongPackMaskItem")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksDropdown_SongPackMaskItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub localizedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub order: i32,
    pub songPackMask: crate::GlobalNamespace::SongPackMask,
}
#[cfg(feature = "QuickPlaySongPacksDropdown+SongPackMaskItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem => ""
    ."QuickPlaySongPacksDropdown/SongPackMaskItem"
);
#[cfg(feature = "QuickPlaySongPacksDropdown+SongPackMaskItem")]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown+SongPackMaskItem")]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown+SongPackMaskItem")]
impl crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "QuickPlaySongPacksDropdown+SongPackMaskItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
