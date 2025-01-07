#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClipAsyncLoaderExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AudioClipAsyncLoaderExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AudioClipAsyncLoaderExtensions";
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
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::AudioClipAsyncLoaderExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioClipAsyncLoaderExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
impl crate::GlobalNamespace::AudioClipAsyncLoaderExtensions {
    pub fn LoadPreview(
        asyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadPreview", (asyncLoader, beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSong(
        asyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSong", (asyncLoader, beatmapLevelData))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadPreview(
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadPreview", (_cordl__, beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSong(
        asyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSong", (asyncLoader, beatmapLevelData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AudioClipAsyncLoaderExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
