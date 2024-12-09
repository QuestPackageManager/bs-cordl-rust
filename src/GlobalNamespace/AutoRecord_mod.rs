#[cfg(feature = "AutoRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoRecord {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AutoRecord")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AutoRecord => ""."AutoRecord"
);
#[cfg(feature = "AutoRecord")]
impl std::ops::Deref for crate::GlobalNamespace::AutoRecord {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AutoRecord")]
impl std::ops::DerefMut for crate::GlobalNamespace::AutoRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AutoRecord")]
impl crate::GlobalNamespace::AutoRecord {
    #[cfg(feature = "AutoRecord+Beatmap")]
    pub type Beatmap = crate::GlobalNamespace::AutoRecord_Beatmap;
}
#[cfg(feature = "AutoRecord")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AutoRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AutoRecord+Beatmap")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AutoRecord_Beatmap {
    pub beatsPerMinute: f32,
    pub noteLineCount: i32,
    pub items: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapDataItem,
    >,
}
#[cfg(feature = "AutoRecord+Beatmap")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AutoRecord_Beatmap => ""
    ."AutoRecord/Beatmap"
);
#[cfg(feature = "AutoRecord+Beatmap")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::AutoRecord_Beatmap {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "AutoRecord+Beatmap")]
impl crate::GlobalNamespace::AutoRecord_Beatmap {}
