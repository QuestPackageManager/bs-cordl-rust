#[cfg(feature = "cordl_class_BeatmapSaveDataVersion4+BeatmapSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub colorNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatmapBeatIndex>,
        >,
    >,
    pub bombNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatmapBeatIndex>,
        >,
    >,
    pub obstacles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatmapBeatIndex>,
        >,
    >,
    pub chains: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::ChainBeatIndex>,
        >,
    >,
    pub arcs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::ArcBeatIndex>,
        >,
    >,
    pub colorNotesData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::ColorNote>,
    >,
    pub bombNotesData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::BombNote>,
    >,
    pub obstaclesData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Obstacle>,
    >,
    pub chainsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Chain>,
    >,
    pub arcsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Arc>,
    >,
    pub njsEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatIndex>,
        >,
    >,
    pub njsEventData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::NoteJumpMovementSpeedEvent,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion4+BeatmapSaveData")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatmapSaveData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
impl crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    pub const kCurrentVersion: &'static str = "4.1.0";
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion4+BeatmapSaveData")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
