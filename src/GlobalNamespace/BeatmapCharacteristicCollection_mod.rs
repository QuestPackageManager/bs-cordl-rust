#[cfg(feature = "BeatmapCharacteristicCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicCollection {
    __cordl_parent: crate::System::Object,
    pub _beatmapCharacteristicsBySerializedName: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut BeatmapCharacteristicSO,
    >,
    pub beatmapCharacteristics: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut BeatmapCharacteristicSO,
    >,
    pub disabledBeatmapCharacteristics: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut BeatmapCharacteristicSO,
    >,
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapCharacteristicCollection => ""
    ."BeatmapCharacteristicCollection"
);
#[cfg(feature = "BeatmapCharacteristicCollection")]
impl std::ops::Deref for BeatmapCharacteristicCollection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
impl std::ops::DerefMut for BeatmapCharacteristicCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
impl BeatmapCharacteristicCollection {
    pub fn GetBeatmapCharacteristicBySerializedName(
        &mut self,
        serializedName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapCharacteristicSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapCharacteristicSO = __cordl_object
            .invoke("GetBeatmapCharacteristicBySerializedName", (serializedName))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        collection: *mut BeatmapCharacteristicCollectionSO,
        appStaticSettings: *mut AppStaticSettingsSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collection, appStaticSettings))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        collection: *mut BeatmapCharacteristicCollectionSO,
        appStaticSettings: *mut AppStaticSettingsSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collection, appStaticSettings))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
impl quest_hook::libil2cpp::ObjectType for BeatmapCharacteristicCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
