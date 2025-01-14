#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBaseData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub b: f32,
    pub i: crate::BeatmapSaveDataVersion3::TransitionType,
    pub c: crate::BeatmapSaveDataCommon::EnvironmentColorType,
    pub s: f32,
    pub f: i32,
    pub sb: f32,
    pub sf: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion3::LightColorBaseData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "LightColorBaseData";
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
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightColorBaseData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightColorBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
impl crate::BeatmapSaveDataVersion3::LightColorBaseData {
    pub fn New(
        beat: f32,
        transitionType: crate::BeatmapSaveDataVersion3::TransitionType,
        colorType: crate::BeatmapSaveDataCommon::EnvironmentColorType,
        brightness: f32,
        strobeFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beat,
                    transitionType,
                    colorType,
                    brightness,
                    strobeFrequency,
                    strobeBrightness,
                    strobeFade,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        transitionType: crate::BeatmapSaveDataVersion3::TransitionType,
        colorType: crate::BeatmapSaveDataCommon::EnvironmentColorType,
        brightness: f32,
        strobeFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    crate::BeatmapSaveDataVersion3::TransitionType,
                    crate::BeatmapSaveDataCommon::EnvironmentColorType,
                    f32,
                    i32,
                    f32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        beat,
                        transitionType,
                        colorType,
                        brightness,
                        strobeFrequency,
                        strobeBrightness,
                        strobeFade,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_beat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_beat", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_brightness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_brightness")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_brightness", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_colorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::EnvironmentColorType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::BeatmapSaveDataCommon::EnvironmentColorType,
                0usize,
            >("get_colorType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_colorType", 0usize
                )
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::EnvironmentColorType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_strobeBeatFrequency(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_strobeBeatFrequency")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_strobeBeatFrequency", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_strobeBrightness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_strobeBrightness")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_strobeBrightness", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_strobeFade(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_strobeFade")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_strobeFade", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataVersion3::TransitionType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::BeatmapSaveDataVersion3::TransitionType,
                0usize,
            >("get_transitionType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_transitionType", 0usize
                )
            });
        let __cordl_ret: crate::BeatmapSaveDataVersion3::TransitionType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightColorBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
