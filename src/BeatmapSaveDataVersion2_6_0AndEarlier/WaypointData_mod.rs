#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
#[repr(C)]
#[derive(Debug)]
pub struct WaypointData {
    __cordl_parent: crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
    pub _time: f32,
    pub _lineIndex: i32,
    pub _lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    pub _offsetDirection: crate::BeatmapSaveDataCommon::OffsetDirection,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "WaypointData";
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
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    type Target = crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    pub fn New(
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        offsetDirection: crate::BeatmapSaveDataCommon::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, lineIndex, lineLayer, offsetDirection))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        offsetDirection: crate::BeatmapSaveDataCommon::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    i32,
                    crate::BeatmapSaveDataCommon::NoteLineLayer,
                    crate::BeatmapSaveDataCommon::OffsetDirection,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (_cordl_time, lineIndex, lineLayer, offsetDirection),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_lineIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_lineIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_lineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteLineLayer> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::BeatmapSaveDataCommon::NoteLineLayer,
                0usize,
            >("get_lineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_lineLayer", 0usize
                )
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteLineLayer = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_offsetDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::OffsetDirection> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::BeatmapSaveDataCommon::OffsetDirection,
                0usize,
            >("get_offsetDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_offsetDirection", 0usize
                )
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::OffsetDirection = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_time")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_time", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
