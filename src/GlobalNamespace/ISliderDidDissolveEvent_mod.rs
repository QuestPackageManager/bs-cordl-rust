#[cfg(feature = "ISliderDidDissolveEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ISliderDidDissolveEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISliderDidDissolveEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ISliderDidDissolveEvent => ""
    ."ISliderDidDissolveEvent"
);
#[cfg(feature = "ISliderDidDissolveEvent")]
impl std::ops::Deref for crate::GlobalNamespace::ISliderDidDissolveEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISliderDidDissolveEvent")]
impl std::ops::DerefMut for crate::GlobalNamespace::ISliderDidDissolveEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISliderDidDissolveEvent")]
impl crate::GlobalNamespace::ISliderDidDissolveEvent {
    pub fn HandleSliderDidDissolve(
        &mut self,
        sliderController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderDidDissolve", (sliderController))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISliderDidDissolveEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ISliderDidDissolveEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
