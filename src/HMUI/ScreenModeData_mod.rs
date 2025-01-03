#[cfg(feature = "HMUI+ScreenModeData")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenModeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Vector3,
    pub scale: f32,
    pub radius: f32,
    pub offsetHeightByHeadPos: bool,
    pub yOffsetRelativeToHead: f32,
    pub minYPos: f32,
}
#[cfg(feature = "HMUI+ScreenModeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ScreenModeData => "HMUI"."ScreenModeData"
);
#[cfg(feature = "HMUI+ScreenModeData")]
impl std::ops::Deref for crate::HMUI::ScreenModeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScreenModeData")]
impl std::ops::DerefMut for crate::HMUI::ScreenModeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScreenModeData")]
impl crate::HMUI::ScreenModeData {
    pub fn New(
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Vector3,
        scale: f32,
        radius: f32,
        offsetHeightByHeadPos: bool,
        yOffsetRelativeToHead: f32,
        minYPos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    position,
                    rotation,
                    scale,
                    radius,
                    offsetHeightByHeadPos,
                    yOffsetRelativeToHead,
                    minYPos,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Vector3,
        scale: f32,
        radius: f32,
        offsetHeightByHeadPos: bool,
        yOffsetRelativeToHead: f32,
        minYPos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    position,
                    rotation,
                    scale,
                    radius,
                    offsetHeightByHeadPos,
                    yOffsetRelativeToHead,
                    minYPos,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ScreenModeData")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ScreenModeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
