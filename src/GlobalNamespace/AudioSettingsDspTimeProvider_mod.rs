#[cfg(feature = "AudioSettingsDspTimeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSettingsDspTimeProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AudioSettingsDspTimeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioSettingsDspTimeProvider =>
    ""."AudioSettingsDspTimeProvider"
);
#[cfg(feature = "AudioSettingsDspTimeProvider")]
impl std::ops::Deref for crate::GlobalNamespace::AudioSettingsDspTimeProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioSettingsDspTimeProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioSettingsDspTimeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioSettingsDspTimeProvider")]
impl crate::GlobalNamespace::AudioSettingsDspTimeProvider {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dspTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_dspTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioSettingsDspTimeProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AudioSettingsDspTimeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AudioSettingsDspTimeProvider")]
impl AsRef<crate::GlobalNamespace::IDspTimeProvider>
for crate::GlobalNamespace::AudioSettingsDspTimeProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::IDspTimeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AudioSettingsDspTimeProvider")]
impl AsMut<crate::GlobalNamespace::IDspTimeProvider>
for crate::GlobalNamespace::AudioSettingsDspTimeProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IDspTimeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
