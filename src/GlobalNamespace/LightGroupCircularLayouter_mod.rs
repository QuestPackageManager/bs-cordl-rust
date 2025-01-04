#[cfg(feature = "LightGroupCircularLayouter")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupCircularLayouter {
    __cordl_parent: crate::GlobalNamespace::LightGroupSubsystem,
    pub _radius: f32,
    pub _angle: f32,
    pub _startingAngle: f32,
    pub _rotationDirection: crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection,
    pub _staticRotation: bool,
    pub _additionalAngle: crate::UnityEngine::Vector3,
}
#[cfg(feature = "LightGroupCircularLayouter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightGroupCircularLayouter =>
    ""."LightGroupCircularLayouter"
);
#[cfg(feature = "LightGroupCircularLayouter")]
impl std::ops::Deref for crate::GlobalNamespace::LightGroupCircularLayouter {
    type Target = crate::GlobalNamespace::LightGroupSubsystem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupCircularLayouter")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightGroupCircularLayouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupCircularLayouter")]
impl crate::GlobalNamespace::LightGroupCircularLayouter {
    #[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
    pub type RotationDirection = crate::GlobalNamespace::LightGroupCircularLayouter_RotationDirection;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightGroupCircularLayouter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightGroupCircularLayouter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightGroupCircularLayouter_RotationDirection {
    #[default]
    Clockwise = 0i32,
    Counterclockwise = 1i32,
}
#[cfg(feature = "LightGroupCircularLayouter+RotationDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightGroupCircularLayouter_RotationDirection => ""
    ."LightGroupCircularLayouter/RotationDirection"
);
