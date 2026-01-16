#[cfg(feature = "cordl_class_LightRotationBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationBaseData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub beat: f32,
    pub usePreviousEventRotationValue: bool,
    pub easeType: crate::GlobalNamespace::EaseType,
    pub rotation: f32,
    pub loopsCount: i32,
    pub rotationDirection: crate::GlobalNamespace::LightRotationDirection,
}
#[cfg(feature = "cordl_class_LightRotationBaseData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::LightRotationBaseData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightRotationBaseData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "LightRotationBaseData")]
impl std::ops::Deref for crate::GlobalNamespace::LightRotationBaseData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBaseData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightRotationBaseData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBaseData")]
impl crate::GlobalNamespace::LightRotationBaseData {
    pub fn New(
        beat: f32,
        usePreviousEventRotationValue: bool,
        easeType: crate::GlobalNamespace::EaseType,
        rotation: f32,
        loopsCount: i32,
        rotationDirection: crate::GlobalNamespace::LightRotationDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                beat,
                usePreviousEventRotationValue,
                easeType,
                rotation,
                loopsCount,
                rotationDirection,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousEventRotationValue: bool,
        easeType: crate::GlobalNamespace::EaseType,
        rotation: f32,
        loopsCount: i32,
        rotationDirection: crate::GlobalNamespace::LightRotationDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        f32,
                        bool,
                        crate::GlobalNamespace::EaseType,
                        f32,
                        i32,
                        crate::GlobalNamespace::LightRotationDirection,
                    ), quest_hook::libil2cpp::Void, 6usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    beat,
                    usePreviousEventRotationValue,
                    easeType,
                    rotation,
                    loopsCount,
                    rotationDirection,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LightRotationBaseData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LightRotationBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
