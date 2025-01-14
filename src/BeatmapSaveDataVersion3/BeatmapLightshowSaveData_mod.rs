#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapLightshowSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLightshowSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub basicBeatmapEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::BasicEventData>,
        >,
    >,
    pub colorBoostBeatmapEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion3::ColorBoostEventData,
            >,
        >,
    >,
    pub lightColorEventBoxGroups: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
            >,
        >,
    >,
    pub lightRotationEventBoxGroups: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion3::LightRotationEventBoxGroup,
            >,
        >,
    >,
    pub lightTranslationEventBoxGroups: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
            >,
        >,
    >,
    pub vfxEventBoxGroups: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::FxEventBoxGroup>,
        >,
    >,
    pub _fxEventsCollection: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion3::FxEventsCollection,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapLightshowSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion3::BeatmapLightshowSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "BeatmapLightshowSaveData";
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
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapLightshowSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::BeatmapLightshowSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapLightshowSaveData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::BeatmapLightshowSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapLightshowSaveData")]
impl crate::BeatmapSaveDataVersion3::BeatmapLightshowSaveData {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isEmpty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isEmpty", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapLightshowSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::BeatmapLightshowSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
