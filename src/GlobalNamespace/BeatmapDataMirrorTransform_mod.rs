#[cfg(feature = "BeatmapDataMirrorTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataMirrorTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataMirrorTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDataMirrorTransform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDataMirrorTransform";
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
#[cfg(feature = "BeatmapDataMirrorTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataMirrorTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataMirrorTransform")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataMirrorTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataMirrorTransform")]
impl crate::GlobalNamespace::BeatmapDataMirrorTransform {
    pub fn CreateTransformedData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTransformedData", (beatmapData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataMirrorTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataMirrorTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
