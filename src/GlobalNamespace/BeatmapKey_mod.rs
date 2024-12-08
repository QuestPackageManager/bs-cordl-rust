#[cfg(feature = "BeatmapKey")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapKey {
    pub beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    pub difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub levelId: *mut crate::System::String,
}
#[cfg(feature = "BeatmapKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapKey => ""."BeatmapKey"
);
#[cfg(feature = "BeatmapKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::BeatmapKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapKey")]
impl crate::GlobalNamespace::BeatmapKey {
    pub fn Equals_BeatmapKey0(
        &mut self,
        other: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    > {
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapKeyNetSerializable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetIdentifier",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SerializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SerializedName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        levelId: *mut crate::System::String,
        beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (levelId, beatmapCharacteristic, difficulty),
        )?;
        Ok(__cordl_ret)
    }
}
