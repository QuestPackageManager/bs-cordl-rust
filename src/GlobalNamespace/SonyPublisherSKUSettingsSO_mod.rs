#[cfg(feature = "SonyPublisherSKUSettingsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyPublisherSKUSettingsSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _skuName: *mut crate::System::String,
    pub _serviceIdPrefix: *mut crate::System::String,
    pub _titleId: *mut crate::System::String,
    pub _productLabel: *mut crate::System::String,
}
#[cfg(feature = "SonyPublisherSKUSettingsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SonyPublisherSKUSettingsSO =>
    ""."SonyPublisherSKUSettingsSO"
);
#[cfg(feature = "SonyPublisherSKUSettingsSO")]
impl std::ops::Deref for crate::GlobalNamespace::SonyPublisherSKUSettingsSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyPublisherSKUSettingsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyPublisherSKUSettingsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyPublisherSKUSettingsSO")]
impl crate::GlobalNamespace::SonyPublisherSKUSettingsSO {
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
    pub fn get_productLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_productLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_serviceIdPrefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_serviceIdPrefix", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_skuName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_skuName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_titleId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_titleId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyPublisherSKUSettingsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyPublisherSKUSettingsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
