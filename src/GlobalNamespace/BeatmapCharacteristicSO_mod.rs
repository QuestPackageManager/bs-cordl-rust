#[cfg(feature = "BeatmapCharacteristicSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _icon: *mut crate::UnityEngine::Sprite,
    pub _descriptionLocalizationKey: *mut crate::System::String,
    pub _characteristicNameLocalizationKey: *mut crate::System::String,
    pub _serializedName: *mut crate::System::String,
    pub _compoundIdPartName: *mut crate::System::String,
    pub _sortingOrder: i32,
    pub _containsRotationEvents: bool,
    pub _requires360Movement: bool,
    pub _numberOfColors: i32,
}
#[cfg(feature = "BeatmapCharacteristicSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapCharacteristicSO => ""
    ."BeatmapCharacteristicSO"
);
#[cfg(feature = "BeatmapCharacteristicSO")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapCharacteristicSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapCharacteristicSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicSO")]
impl crate::GlobalNamespace::BeatmapCharacteristicSO {
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
    pub fn get_characteristicNameLocalizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_characteristicNameLocalizationKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_compoundIdPartName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_compoundIdPartName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_containsRotationEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_containsRotationEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_descriptionLocalizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_descriptionLocalizationKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_icon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_icon", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_numberOfColors(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfColors", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_requires360Movement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_requires360Movement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_serializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_serializedName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sortingOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sortingOrder", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapCharacteristicSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCharacteristicSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
