#[cfg(feature = "PS5PublisherSKUSettingsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5PublisherSKUSettingsSO {
    __cordl_parent: SonyPublisherSKUSettingsSO,
}
#[cfg(feature = "PS5PublisherSKUSettingsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PS5PublisherSKUSettingsSO => ""
    ."PS5PublisherSKUSettingsSO"
);
#[cfg(feature = "PS5PublisherSKUSettingsSO")]
impl std::ops::Deref for PS5PublisherSKUSettingsSO {
    type Target = SonyPublisherSKUSettingsSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5PublisherSKUSettingsSO")]
impl std::ops::DerefMut for PS5PublisherSKUSettingsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5PublisherSKUSettingsSO")]
impl PS5PublisherSKUSettingsSO {
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
#[cfg(feature = "PS5PublisherSKUSettingsSO")]
impl quest_hook::libil2cpp::ObjectType for PS5PublisherSKUSettingsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
