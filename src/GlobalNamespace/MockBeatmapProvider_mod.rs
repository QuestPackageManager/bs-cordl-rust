#[cfg(feature = "MockBeatmapProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MockBeatmapProvider")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::MockBeatmapProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockBeatmapProvider";
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
#[cfg(feature = "MockBeatmapProvider")]
impl std::ops::Deref for crate::GlobalNamespace::MockBeatmapProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockBeatmapProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl crate::GlobalNamespace::MockBeatmapProvider {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks(
        &mut self,
        playerCount: i32,
        suggestedBeatmaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapKeyNetSerializable,
                >,
            >,
        >,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        ownedSongPacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::GlobalNamespace::SongPackMask,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapKeyNetSerializable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self,
                    "SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (playerCount, suggestedBeatmaps, selectionMask, ownedSongPacks),
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "VerifyBeatmapForSelectionMask", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (beatmapKeySerializable, selectionMask))
        };
        Ok(__cordl_ret.into())
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
}
#[cfg(feature = "MockBeatmapProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MockBeatmapProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl AsRef<crate::GlobalNamespace::IServerBeatmapProvider>
for crate::GlobalNamespace::MockBeatmapProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::IServerBeatmapProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl AsMut<crate::GlobalNamespace::IServerBeatmapProvider>
for crate::GlobalNamespace::MockBeatmapProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IServerBeatmapProvider {
        unsafe { std::mem::transmute(self) }
    }
}
