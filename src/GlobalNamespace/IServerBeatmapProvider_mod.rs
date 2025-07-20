#[cfg(feature = "IServerBeatmapProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IServerBeatmapProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IServerBeatmapProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IServerBeatmapProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IServerBeatmapProvider";
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
#[cfg(feature = "IServerBeatmapProvider")]
impl std::ops::Deref for crate::GlobalNamespace::IServerBeatmapProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::IServerBeatmapProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl crate::GlobalNamespace::IServerBeatmapProvider {
    pub fn SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks(
        &mut self,
        playerCount: i32,
        beatmapsSuggestedByPlayers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapKeyNetSerializable,
                >,
            >,
        >,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        playerOwnedSongPacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::GlobalNamespace::SongPackMask,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapKeyNetSerializable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapKeyNetSerializable,
                                    >,
                                >,
                            >,
                            crate::GlobalNamespace::BeatmapLevelSelectionMask,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                    crate::GlobalNamespace::SongPackMask,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapKeyNetSerializable,
                        >,
                        4usize,
                    >("SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        playerCount,
                        beatmapsSuggestedByPlayers,
                        selectionMask,
                        playerOwnedSongPacks,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VerifyBeatmapForSelectionMask(
        &mut self,
        beatmapKeySerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapKeyNetSerializable,
                            >,
                            crate::GlobalNamespace::BeatmapLevelSelectionMask,
                        ),
                        bool,
                        2usize,
                    >("VerifyBeatmapForSelectionMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VerifyBeatmapForSelectionMask", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (beatmapKeySerializable, selectionMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IServerBeatmapProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
