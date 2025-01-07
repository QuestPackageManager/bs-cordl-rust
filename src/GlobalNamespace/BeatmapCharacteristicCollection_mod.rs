#[cfg(feature = "BeatmapCharacteristicCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapCharacteristicsBySerializedName: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
    pub beatmapCharacteristics: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
    pub disabledBeatmapCharacteristics: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapCharacteristicCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapCharacteristicCollection";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapCharacteristicCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapCharacteristicCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
impl crate::GlobalNamespace::BeatmapCharacteristicCollection {
    pub fn GetBeatmapCharacteristicBySerializedName(
        &mut self,
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        > = __cordl_object
            .invoke("GetBeatmapCharacteristicBySerializedName", (serializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        collection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollectionSO,
        >,
        appStaticSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AppStaticSettingsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collection, appStaticSettings))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        collection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollectionSO,
        >,
        appStaticSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AppStaticSettingsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collection, appStaticSettings))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapCharacteristicCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCharacteristicCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
