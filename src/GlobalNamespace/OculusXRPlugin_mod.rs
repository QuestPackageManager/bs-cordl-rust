#[cfg(feature = "OculusXRPlugin")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusXRPlugin {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OculusXRPlugin")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OculusXRPlugin {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusXRPlugin";
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
#[cfg(feature = "OculusXRPlugin")]
impl std::ops::Deref for crate::GlobalNamespace::OculusXRPlugin {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusXRPlugin")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusXRPlugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusXRPlugin")]
impl crate::GlobalNamespace::OculusXRPlugin {
    pub fn SetAppSpacePosition(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAppSpacePosition", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAppSpaceRotation(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAppSpaceRotation", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorOffset(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColorOffset", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorScale(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColorScale", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDepthSubmission(
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDepthSubmission", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSpaceWarp(
        on: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSpaceWarp", (on))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusXRPlugin")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OculusXRPlugin {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
