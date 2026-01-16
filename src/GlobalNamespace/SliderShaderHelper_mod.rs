#[cfg(feature = "cordl_class_SliderShaderHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderShaderHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_SliderShaderHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SliderShaderHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SliderShaderHelper";
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
#[cfg(feature = "SliderShaderHelper")]
impl std::ops::Deref for crate::GlobalNamespace::SliderShaderHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderShaderHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderShaderHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderShaderHelper")]
impl crate::GlobalNamespace::SliderShaderHelper {
    pub fn EnableSaberAttraction(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        enableSaberAttraction: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "EnableSaberAttraction"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableSaberAttraction",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (materialPropertyBlock, enableSaberAttraction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetColor(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        crate::UnityEngine::Color,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetColor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (materialPropertyBlock, color))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetHeadNoteJump(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        halfJumpDuration: f32,
        headNoteGravity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetHeadNoteJump")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetHeadNoteJump",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (materialPropertyBlock, halfJumpDuration, headNoteGravity),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInitialProperties_Color_f32_f32__cordl_bool__cordl_bool_f32_0(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        sliderColor: crate::UnityEngine::Color,
        sliderZLength: f32,
        sliderLength: f32,
        hasHeadNote: bool,
        hasTailNote: bool,
        randomValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        crate::UnityEngine::Color,
                        f32,
                        f32,
                        bool,
                        bool,
                        f32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "SetInitialProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInitialProperties",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    materialPropertyBlock,
                    sliderColor,
                    sliderZLength,
                    sliderLength,
                    hasHeadNote,
                    hasTailNote,
                    randomValue,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInitialProperties_SliderController1(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        sliderController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderController>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetInitialProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInitialProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (materialPropertyBlock, sliderController))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetJumpSpeedAndDistance(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        noteJumpSpeed: f32,
        noteJumpDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetJumpSpeedAndDistance"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetJumpSpeedAndDistance",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (materialPropertyBlock, noteJumpSpeed, noteJumpDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNoteJump(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        propertyId: i32,
        halfJumpDuration: f32,
        noteGravity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        i32,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetNoteJump")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetNoteJump",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    materialPropertyBlock,
                    propertyId,
                    halfJumpDuration,
                    noteGravity,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSaberAttractionPoint(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        attractPoint: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetSaberAttractionPoint"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetSaberAttractionPoint",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (materialPropertyBlock, attractPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTailHeadNoteJumpOffsetDifference(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        tailHeadNoteJumpOffsetDifference: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetTailHeadNoteJumpOffsetDifference"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetTailHeadNoteJumpOffsetDifference",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (materialPropertyBlock, tailHeadNoteJumpOffsetDifference),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTailNoteJump(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        halfJumpDuration: f32,
        headNoteGravity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetTailNoteJump")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetTailNoteJump",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (materialPropertyBlock, halfJumpDuration, headNoteGravity),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTimeSinceHeadNoteJump(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetTimeSinceHeadNoteJump"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetTimeSinceHeadNoteJump",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (materialPropertyBlock, _cordl_time))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMaterialPropertyBlock(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        sliderController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderController>,
        variableMovementDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVariableMovementDataProvider,
        >,
        jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IJumpOffsetYProvider,
        >,
        timeHelper: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TimeHelper>,
        timeSinceHeadNoteJump: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderController>,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IVariableMovementDataProvider,
                        >,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IJumpOffsetYProvider>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TimeHelper>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "UpdateMaterialPropertyBlock"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateMaterialPropertyBlock",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    materialPropertyBlock,
                    sliderController,
                    variableMovementDataProvider,
                    jumpOffsetYProvider,
                    timeHelper,
                    timeSinceHeadNoteJump,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_SliderShaderHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SliderShaderHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
