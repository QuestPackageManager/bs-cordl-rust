#[cfg(feature = "MenuLightsPresetSO+LightIdColorPair")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuLightsPresetSO_LightIdColorPair {
    __cordl_parent: crate::System::Object,
    pub lightId: i32,
    pub baseColor: *mut ColorSO,
    pub intensity: f32,
}
#[cfg(feature = "MenuLightsPresetSO+LightIdColorPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair => ""
    ."MenuLightsPresetSO/LightIdColorPair"
);
#[cfg(feature = "MenuLightsPresetSO+LightIdColorPair")]
impl std::ops::Deref for crate::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuLightsPresetSO+LightIdColorPair")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuLightsPresetSO+LightIdColorPair")]
impl crate::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair {
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
    pub fn get_lightColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_lightColor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MenuLightsPresetSO+LightIdColorPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuLightsPresetSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuLightsPresetSO {
    __cordl_parent: PersistentScriptableObject,
    pub _playersPlaceNeonsColor: *mut ColorSO,
    pub _playersPlaceNeonsIntensity: f32,
    pub _lightIdColorPairs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair,
    >,
}
#[cfg(feature = "MenuLightsPresetSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MenuLightsPresetSO => ""."MenuLightsPresetSO"
);
#[cfg(feature = "MenuLightsPresetSO")]
impl std::ops::Deref for MenuLightsPresetSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuLightsPresetSO")]
impl std::ops::DerefMut for MenuLightsPresetSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuLightsPresetSO")]
impl MenuLightsPresetSO {
    #[cfg(feature = "MenuLightsPresetSO+LightIdColorPair")]
    pub type LightIdColorPair = crate::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair;
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
    pub fn get_lightIdColorPairs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MenuLightsPresetSO_LightIdColorPair,
        > = __cordl_object.invoke("get_lightIdColorPairs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playersPlaceNeonsColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ColorSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ColorSO = __cordl_object
            .invoke("get_playersPlaceNeonsColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playersPlaceNeonsIntensity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_playersPlaceNeonsIntensity", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MenuLightsPresetSO")]
impl quest_hook::libil2cpp::ObjectType for MenuLightsPresetSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}