#[cfg(feature = "LightRotationBaseData")]
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
#[cfg(feature = "LightRotationBaseData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightRotationBaseData => ""
    ."LightRotationBaseData"
);
#[cfg(feature = "LightRotationBaseData")]
impl std::ops::Deref for crate::GlobalNamespace::LightRotationBaseData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBaseData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightRotationBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
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
        Ok(__cordl_object)
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LightRotationBaseData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightRotationBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
