#[cfg(feature = "cordl_class_LeaderboardUploadData")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct LeaderboardUploadData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub score: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub authorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub bpm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub infoHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub modifiers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "cordl_class_LeaderboardUploadData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::LeaderboardUploadData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LeaderboardUploadData";
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
#[cfg(feature = "LeaderboardUploadData")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardUploadData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardUploadData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardUploadData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardUploadData")]
impl crate::GlobalNamespace::LeaderboardUploadData {
    pub fn New(
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        authorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bpm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        infoHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        modifiers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                playerName,
                playerId,
                score,
                leaderboardId,
                songName,
                songSubName,
                authorName,
                bpm,
                difficulty,
                infoHash,
                modifiers,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        authorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bpm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        infoHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        modifiers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 11usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    playerName,
                    playerId,
                    score,
                    leaderboardId,
                    songName,
                    songSubName,
                    authorName,
                    bpm,
                    difficulty,
                    infoHash,
                    modifiers,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LeaderboardUploadData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LeaderboardUploadData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
