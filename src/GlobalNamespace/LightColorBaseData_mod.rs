#[cfg(feature = "LightColorBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBaseData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub beat: f32,
    pub usePreviousValue: bool,
    pub easeType: crate::GlobalNamespace::EaseType,
    pub colorType: crate::GlobalNamespace::EnvironmentColorType,
    pub brightness: f32,
    pub strobeBeatFrequency: i32,
    pub strobeBrightness: f32,
    pub strobeFade: bool,
}
#[cfg(feature = "LightColorBaseData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::LightColorBaseData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
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
#[cfg(feature = "LightColorBaseData")]
impl std::ops::Deref for crate::GlobalNamespace::LightColorBaseData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBaseData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightColorBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBaseData")]
impl crate::GlobalNamespace::LightColorBaseData {
    pub fn New(
        beat: f32,
        usePreviousValue: bool,
        easeType: crate::GlobalNamespace::EaseType,
        colorType: crate::GlobalNamespace::EnvironmentColorType,
        brightness: f32,
        strobeBeatFrequency: i32,
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
                    usePreviousValue,
                    easeType,
                    colorType,
                    brightness,
                    strobeBeatFrequency,
                    strobeBrightness,
                    strobeFade,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousValue: bool,
        easeType: crate::GlobalNamespace::EaseType,
        colorType: crate::GlobalNamespace::EnvironmentColorType,
        brightness: f32,
        strobeBeatFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    bool,
                    crate::GlobalNamespace::EaseType,
                    crate::GlobalNamespace::EnvironmentColorType,
                    f32,
                    i32,
                    f32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        beat,
                        usePreviousValue,
                        easeType,
                        colorType,
                        brightness,
                        strobeBeatFrequency,
                        strobeBrightness,
                        strobeFade,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightColorBaseData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LightColorBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
