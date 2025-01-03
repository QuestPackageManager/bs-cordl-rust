#[cfg(feature = "SliderShaderHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderShaderHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SliderShaderHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderShaderHelper => ""
    ."SliderShaderHelper"
);
#[cfg(feature = "SliderShaderHelper")]
impl std::ops::Deref for crate::GlobalNamespace::SliderShaderHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderShaderHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderShaderHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderShaderHelper")]
impl crate::GlobalNamespace::SliderShaderHelper {
    pub fn EnableSaberAttraction(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        enableSaberAttraction: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EnableSaberAttraction",
                (materialPropertyBlock, enableSaberAttraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColor(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColor", (materialPropertyBlock, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHeadNoteJump(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        halfJumpDuration: f32,
        headNoteGravity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetHeadNoteJump",
                (materialPropertyBlock, halfJumpDuration, headNoteGravity),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInitialProperties_Color_f32_f32__cordl_bool__cordl_bool_f32_0(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        sliderColor: crate::UnityEngine::Color,
        sliderZLength: f32,
        sliderLength: f32,
        hasHeadNote: bool,
        hasTailNote: bool,
        randomValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetInitialProperties",
                (
                    materialPropertyBlock,
                    sliderColor,
                    sliderZLength,
                    sliderLength,
                    hasHeadNote,
                    hasTailNote,
                    randomValue,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInitialProperties_SliderController1(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        sliderController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInitialProperties", (materialPropertyBlock, sliderController))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetJumpSpeedAndDistance(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        noteJumpSpeed: f32,
        noteJumpDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetJumpSpeedAndDistance",
                (materialPropertyBlock, noteJumpSpeed, noteJumpDistance),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNoteJump(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        propertyId: i32,
        halfJumpDuration: f32,
        noteGravity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetNoteJump",
                (materialPropertyBlock, propertyId, halfJumpDuration, noteGravity),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSaberAttractionPoint(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        attractPoint: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSaberAttractionPoint", (materialPropertyBlock, attractPoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTailHeadNoteJumpOffsetDifference(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        tailHeadNoteJumpOffsetDifference: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetTailHeadNoteJumpOffsetDifference",
                (materialPropertyBlock, tailHeadNoteJumpOffsetDifference),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTailNoteJump(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        halfJumpDuration: f32,
        headNoteGravity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetTailNoteJump",
                (materialPropertyBlock, halfJumpDuration, headNoteGravity),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTimeSinceHeadNoteJump(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTimeSinceHeadNoteJump", (materialPropertyBlock, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMaterialPropertyBlock(
        materialPropertyBlock: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        sliderController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
        variableMovementDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVariableMovementDataProvider,
        >,
        jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IJumpOffsetYProvider,
        >,
        timeSinceHeadNoteJump: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UpdateMaterialPropertyBlock",
                (
                    materialPropertyBlock,
                    sliderController,
                    variableMovementDataProvider,
                    jumpOffsetYProvider,
                    timeSinceHeadNoteJump,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SliderShaderHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SliderShaderHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
