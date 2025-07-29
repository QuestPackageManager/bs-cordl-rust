#[cfg(feature = "cordl_class_BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteData {
    __cordl_parent: crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
    pub _time: f32,
    pub _lineIndex: i32,
    pub _lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    pub _type: crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
    pub _cutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "NoteData";
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
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    type Target = crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    pub fn New(
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        _cordl_type: crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
        cutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, lineIndex, lineLayer, _cordl_type, cutDirection),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        _cordl_type: crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
        cutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            f32,
                            i32,
                            crate::BeatmapSaveDataCommon::NoteLineLayer,
                            crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
                            crate::BeatmapSaveDataCommon::NoteCutDirection,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (_cordl_time, lineIndex, lineLayer, _cordl_type, cutDirection),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_cutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::NoteCutDirection,
                        0usize,
                    >("get_cutDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_cutDirection", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteCutDirection = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_lineIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_lineIndex", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteLineLayer> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::NoteLineLayer,
                        0usize,
                    >("get_lineLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_lineLayer", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteLineLayer = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_time")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_time", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
                        0usize,
                    >("get_type")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_type", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
