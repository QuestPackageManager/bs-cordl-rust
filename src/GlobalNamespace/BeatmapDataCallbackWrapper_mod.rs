#[cfg(feature = "BeatmapDataCallbackWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataCallbackWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub BasicBeatmapEventType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub aheadTime: f32,
    pub subtypeIdentifiers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
}
#[cfg(feature = "BeatmapDataCallbackWrapper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDataCallbackWrapper";
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
#[cfg(feature = "BeatmapDataCallbackWrapper")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataCallbackWrapper")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataCallbackWrapper")]
impl crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    pub fn CallCallback(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallback", (beatmapData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        aheadTime: f32,
        BasicBeatmapEventType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        subtypeIdentifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (aheadTime, BasicBeatmapEventType, subtypeIdentifiers),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        aheadTime: f32,
        BasicBeatmapEventType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        subtypeIdentifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aheadTime, BasicBeatmapEventType, subtypeIdentifiers))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataCallbackWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
