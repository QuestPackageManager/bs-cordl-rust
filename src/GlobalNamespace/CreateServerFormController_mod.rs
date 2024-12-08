#[cfg(feature = "CreateServerFormController")]
#[repr(C)]
#[derive(Debug)]
pub struct CreateServerFormController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _maxPlayersList: *mut FormattedFloatListSettingsController,
    pub _netDiscoverable: bool,
}
#[cfg(feature = "CreateServerFormController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CreateServerFormController => ""
    ."CreateServerFormController"
);
#[cfg(feature = "CreateServerFormController")]
impl std::ops::Deref for CreateServerFormController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreateServerFormController")]
impl std::ops::DerefMut for CreateServerFormController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreateServerFormController")]
impl CreateServerFormController {
    pub const kMaxPlayers: i32 = 5i32;
    pub const kMinPlayers: i32 = 2i32;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Setup(
        &mut self,
        selectedNumberOfPlayers: i32,
        netDiscoverable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (selectedNumberOfPlayers, netDiscoverable))?;
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
    pub fn get_formData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<CreateServerFormData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: CreateServerFormData = __cordl_object
            .invoke("get_formData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "CreateServerFormController")]
impl quest_hook::libil2cpp::ObjectType for CreateServerFormController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}