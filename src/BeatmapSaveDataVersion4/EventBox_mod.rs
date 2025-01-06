#[cfg(feature = "BeatmapSaveDataVersion4+EventBox")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventBox {
    pub f: i32,
    pub e: i32,
    pub l: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatIndex>,
        >,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBox")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::EventBox =>
    "BeatmapSaveDataVersion4"."EventBox"
);
#[cfg(feature = "BeatmapSaveDataVersion4+EventBox")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::EventBox {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBox")]
impl crate::BeatmapSaveDataVersion4::EventBox {}
