#[cfg(feature = "LightIdColorPair")]
#[repr(C)]
#[derive(Debug)]
pub struct LightIdColorPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lightId: i32,
    pub _useScriptableObjectColor: bool,
    pub color: crate::UnityEngine::Color,
    pub baseColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub intensity: f32,
}
#[cfg(feature = "LightIdColorPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightIdColorPair => ""
    ."LightIdColorPair"
);
#[cfg(feature = "LightIdColorPair")]
impl std::ops::Deref for crate::GlobalNamespace::LightIdColorPair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightIdColorPair")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightIdColorPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightIdColorPair")]
impl crate::GlobalNamespace::LightIdColorPair {
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
    pub fn get_lightColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_lightColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useScriptableObjectColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useScriptableObjectColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useScriptableObjectColor(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useScriptableObjectColor", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightIdColorPair")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LightIdColorPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
