#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapIdentifierNetSerializableHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapIdentifierNetSerializableHelper => ""
    ."BeatmapIdentifierNetSerializableHelper"
);
#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
impl crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    pub fn ToBeatmapKey(
        beatmapKeySerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ToBeatmapKey",
                (beatmapKeySerializable, beatmapCharacteristicCollection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToIdentifier(
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapKeyNetSerializable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToIdentifier", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
