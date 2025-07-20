#[cfg(feature = "NoteBasicCutInfoHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteBasicCutInfoHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteBasicCutInfoHelper";
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
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl std::ops::Deref for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            crate::GlobalNamespace::ColorType,
                            crate::GlobalNamespace::NoteCutDirection,
                            crate::GlobalNamespace::SaberType,
                            f32,
                            crate::UnityEngine::Vector3,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("GetBasicCutInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetBasicCutInfo", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
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
