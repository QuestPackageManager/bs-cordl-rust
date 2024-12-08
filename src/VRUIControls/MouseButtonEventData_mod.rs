#[cfg(feature = "VRUIControls+MouseButtonEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseButtonEventData {
    __cordl_parent: crate::System::Object,
    pub buttonState: crate::UnityEngine::EventSystems::PointerEventData_FramePressState,
    pub buttonData: *mut crate::UnityEngine::EventSystems::PointerEventData,
}
#[cfg(feature = "VRUIControls+MouseButtonEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::VRUIControls::MouseButtonEventData =>
    "VRUIControls"."MouseButtonEventData"
);
#[cfg(feature = "VRUIControls+MouseButtonEventData")]
impl std::ops::Deref for crate::VRUIControls::MouseButtonEventData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+MouseButtonEventData")]
impl std::ops::DerefMut for crate::VRUIControls::MouseButtonEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+MouseButtonEventData")]
impl crate::VRUIControls::MouseButtonEventData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PressedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PressedThisFrame", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReleasedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleasedThisFrame", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "VRUIControls+MouseButtonEventData")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::MouseButtonEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}