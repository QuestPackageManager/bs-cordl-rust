#[cfg(feature = "SmoothCameraSmoothnessSettingsController")]
#[repr(C)]
#[derive(Debug)]
pub struct SmoothCameraSmoothnessSettingsController {
    __cordl_parent: ListSettingsController,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _smoothnesses: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
}
#[cfg(feature = "SmoothCameraSmoothnessSettingsController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SmoothCameraSmoothnessSettingsController => ""
    ."SmoothCameraSmoothnessSettingsController"
);
#[cfg(feature = "SmoothCameraSmoothnessSettingsController")]
impl std::ops::Deref for SmoothCameraSmoothnessSettingsController {
    type Target = ListSettingsController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SmoothCameraSmoothnessSettingsController")]
impl std::ops::DerefMut for SmoothCameraSmoothnessSettingsController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SmoothCameraSmoothnessSettingsController")]
impl SmoothCameraSmoothnessSettingsController {
    pub fn ApplyValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyValue", (idx))?;
        Ok(__cordl_ret)
    }
    pub fn GetInitValues(
        &mut self,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        numberOfElements: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetInitValues", (idx, numberOfElements))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TextForValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("TextForValue", (idx))?;
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
#[cfg(feature = "SmoothCameraSmoothnessSettingsController")]
impl quest_hook::libil2cpp::ObjectType for SmoothCameraSmoothnessSettingsController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}