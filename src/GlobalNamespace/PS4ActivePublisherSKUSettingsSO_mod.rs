#[cfg(feature = "PS4ActivePublisherSKUSettingsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PS4ActivePublisherSKUSettingsSO {
    __cordl_parent: PersistentScriptableObject,
    pub activePublisherSKUSettings: *mut PS4PublisherSKUSettingsSO,
}
#[cfg(feature = "PS4ActivePublisherSKUSettingsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PS4ActivePublisherSKUSettingsSO => ""
    ."PS4ActivePublisherSKUSettingsSO"
);
#[cfg(feature = "PS4ActivePublisherSKUSettingsSO")]
impl std::ops::Deref for PS4ActivePublisherSKUSettingsSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS4ActivePublisherSKUSettingsSO")]
impl std::ops::DerefMut for PS4ActivePublisherSKUSettingsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS4ActivePublisherSKUSettingsSO")]
impl PS4ActivePublisherSKUSettingsSO {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "PS4ActivePublisherSKUSettingsSO")]
impl quest_hook::libil2cpp::ObjectType for PS4ActivePublisherSKUSettingsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
