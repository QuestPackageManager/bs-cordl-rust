#[cfg(feature = "NoteBasicCutInfoHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteBasicCutInfoHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteBasicCutInfoHelper => ""
    ."NoteBasicCutInfoHelper"
);
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl std::ops::Deref for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl crate::GlobalNamespace::NoteBasicCutInfoHelper {
    pub const kMinBladeSpeedForCut: f32 = 2f32;
    pub fn GetBasicCutInfo(
        noteTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        colorType: crate::GlobalNamespace::ColorType,
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        saberType: crate::GlobalNamespace::SaberType,
        saberBladeSpeed: f32,
        cutDirVec: crate::UnityEngine::Vector3,
        cutAngleTolerance: f32,
        directionOK: quest_hook::libil2cpp::ByRefMut<bool>,
        speedOK: quest_hook::libil2cpp::ByRefMut<bool>,
        saberTypeOK: quest_hook::libil2cpp::ByRefMut<bool>,
        cutDirDeviation: quest_hook::libil2cpp::ByRefMut<f32>,
        cutDirAngle: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetBasicCutInfo",
                (
                    noteTransform,
                    colorType,
                    cutDirection,
                    saberType,
                    saberBladeSpeed,
                    cutDirVec,
                    cutAngleTolerance,
                    directionOK,
                    speedOK,
                    saberTypeOK,
                    cutDirDeviation,
                    cutDirAngle,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
