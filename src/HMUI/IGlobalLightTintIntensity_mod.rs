#[cfg(feature = "HMUI+IGlobalLightTintIntensity")]
#[repr(C)]
#[derive(Debug)]
pub struct IGlobalLightTintIntensity {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+IGlobalLightTintIntensity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::IGlobalLightTintIntensity => "HMUI"
    ."IGlobalLightTintIntensity"
);
#[cfg(feature = "HMUI+IGlobalLightTintIntensity")]
impl std::ops::Deref for crate::HMUI::IGlobalLightTintIntensity {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IGlobalLightTintIntensity")]
impl std::ops::DerefMut for crate::HMUI::IGlobalLightTintIntensity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IGlobalLightTintIntensity")]
impl crate::HMUI::IGlobalLightTintIntensity {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_globalLightTintIntensity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_globalLightTintIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_globalLightTintIntensity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_globalLightTintIntensity", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+IGlobalLightTintIntensity")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::IGlobalLightTintIntensity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
