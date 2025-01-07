#[cfg(feature = "NoteCutDirectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutDirectionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "NoteCutDirectionExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteCutDirectionExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutDirectionExtensions";
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
#[cfg(feature = "NoteCutDirectionExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutDirectionExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutDirectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl crate::GlobalNamespace::NoteCutDirectionExtensions {
    pub fn Direction(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Direction", (cutDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMainDirection(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMainDirection", (cutDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOnSamePlane(
        noteCutDirection1: crate::GlobalNamespace::NoteCutDirection,
        noteCutDirection2: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOnSamePlane", (noteCutDirection1, noteCutDirection2))?;
        Ok(__cordl_ret.into())
    }
    pub fn MainNoteCutDirectionFromCutDirAngle(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MainNoteCutDirectionFromCutDirAngle", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mirrored(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mirrored", (cutDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn NoteCutDirectionFromDirection(
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NoteCutDirectionFromDirection", (direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn Opposite(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Opposite", (cutDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rotation(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Rotation", (cutDirection, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotationAngle(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotationAngle", (cutDirection))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutDirectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
