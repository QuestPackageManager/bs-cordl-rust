#[cfg(feature = "BGLib+SaveDataCore+LoaderState")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoaderState {
    FileLoaded = 2u8,
    Loading = 1u8,
    Unloaded = 0u8,
}
#[cfg(feature = "BGLib+SaveDataCore+LoaderState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGLib::SaveDataCore::LoaderState =>
    "BGLib.SaveDataCore"."LoaderState"
);
