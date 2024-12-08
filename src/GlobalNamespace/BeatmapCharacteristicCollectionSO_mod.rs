#[cfg(feature = "BeatmapCharacteristicCollectionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicCollectionSO {
    __cordl_parent: PersistentScriptableObject,
    pub _beatmapCharacteristics: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapCharacteristicSO,
    >,
}
#[cfg(feature = "BeatmapCharacteristicCollectionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapCharacteristicCollectionSO => ""
    ."BeatmapCharacteristicCollectionSO"
);
#[cfg(feature = "BeatmapCharacteristicCollectionSO")]
impl std::ops::Deref for BeatmapCharacteristicCollectionSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicCollectionSO")]
impl std::ops::DerefMut for BeatmapCharacteristicCollectionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicCollectionSO")]
impl BeatmapCharacteristicCollectionSO {
    pub fn get_allBeatmapCharacteristics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            *mut BeatmapCharacteristicSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            *mut BeatmapCharacteristicSO,
        > = __cordl_object.invoke("get_allBeatmapCharacteristics", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapCharacteristicCollectionSO")]
impl quest_hook::libil2cpp::ObjectType for BeatmapCharacteristicCollectionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
