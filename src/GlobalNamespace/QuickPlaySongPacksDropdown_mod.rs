#[cfg(feature = "QuickPlaySongPacksDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksDropdown {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _simpleTextDropdown: *mut crate::HMUI::SimpleTextDropdown,
    pub _songPackMasksModel: *mut SongPackMasksModel,
    pub didSelectCellWithIdxEvent: *mut crate::System::Action_1<i32>,
    pub _initialized: bool,
    pub _quickPlaySongPacksOverride: *mut crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride,
    pub _data: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem,
    >,
}
#[cfg(feature = "QuickPlaySongPacksDropdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for QuickPlaySongPacksDropdown => ""
    ."QuickPlaySongPacksDropdown"
);
#[cfg(feature = "QuickPlaySongPacksDropdown")]
impl std::ops::Deref for QuickPlaySongPacksDropdown {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown")]
impl std::ops::DerefMut for QuickPlaySongPacksDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown")]
impl QuickPlaySongPacksDropdown {
    #[cfg(feature = "QuickPlaySongPacksDropdown+SongPackMaskItem")]
    pub type SongPackMaskItem = crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem;
    #[cfg(feature = "QuickPlaySongPacksDropdown+__c")]
    pub type __c = crate::GlobalNamespace::QuickPlaySongPacksDropdown___c;
    #[cfg(feature = "QuickPlaySongPacksDropdown+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::GlobalNamespace::QuickPlaySongPacksDropdown___c__DisplayClass13_0;
    pub fn GetSelectedSerializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetSelectedSerializedName", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSimpleTextDropdownDidSelectCellWithIdx(
        &mut self,
        dropdownWithTableView: *mut crate::HMUI::DropdownWithTableView,
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
        Ok(__cordl_ret)
    }
    pub fn LazyInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LazyInit", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectCellWithSerializedName(
        &mut self,
        serializedName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectCellWithSerializedName", (serializedName))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverrideSongPacks(
        &mut self,
        quickPlaySongPacksOverride: *mut crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOverrideSongPacks", (quickPlaySongPacksOverride))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn _LazyInit_b__15_0(
        &mut self,
        serializedName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem = __cordl_object
            .invoke("<LazyInit>b__15_0", (serializedName))?;
        Ok(__cordl_ret)
    }
    pub fn _LazyInit_b__15_1(
        &mut self,
        pack: *mut crate::GlobalNamespace::QuickPlaySongPacksOverride_PredefinedPack,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::QuickPlaySongPacksDropdown_SongPackMaskItem = __cordl_object
            .invoke("<LazyInit>b__15_1", (pack))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectCellWithIdxEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectCellWithIdxEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectCellWithIdxEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectCellWithIdxEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "QuickPlaySongPacksDropdown")]
impl quest_hook::libil2cpp::ObjectType for QuickPlaySongPacksDropdown {
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
    __cordl_parent: crate::System::Object,
    pub serializedName: *mut crate::System::String,
    pub localizedName: *mut crate::System::String,
    pub order: i32,
    pub songPackMask: SongPackMask,
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
    type Target = crate::System::Object;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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