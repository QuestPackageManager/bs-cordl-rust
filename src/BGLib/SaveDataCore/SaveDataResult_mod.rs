#[cfg(feature = "BGLib+SaveDataCore+SaveDataResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveDataResult {
    AsyncLoadStateButNoTask = 202i32,
    AttemptedAccessWhileUnloaded = 400i32,
    AttemptedReloadWhileLoading = 401i32,
    CriticalPostLoadStepFailed = 205i32,
    FailedToLoadOrDeserialize = 200i32,
    GenericError = 100i32,
    LoadingNotCompleted = 300i32,
    NoInstanceToSave = 301i32,
    OK = 0i32,
    OK_NoFileLoaded = -2i32,
    OK_NotDirty = -1i32,
    SynchronousLoadAlreadyInLoadingState = 201i32,
    UnknownLoaderState = 101i32,
    UpdateMethodDoesNotExist = 203i32,
    UpdateMethodFailed = 204i32,
}
#[cfg(feature = "BGLib+SaveDataCore+SaveDataResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGLib::SaveDataCore::SaveDataResult =>
    "BGLib.SaveDataCore"."SaveDataResult"
);
