#[cfg(feature = "MenuLightsPresetSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuLightsPresetSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _lightIdColorPairs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::LightIdColorPair>,
    >,
}
#[cfg(feature = "MenuLightsPresetSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuLightsPresetSO => ""
    ."MenuLightsPresetSO"
);
#[cfg(feature = "MenuLightsPresetSO")]
impl std::ops::Deref for crate::GlobalNamespace::MenuLightsPresetSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuLightsPresetSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuLightsPresetSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuLightsPresetSO")]
impl crate::GlobalNamespace::MenuLightsPresetSO {
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
    pub fn get_lightIdColorPairs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::LightIdColorPair,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::LightIdColorPair,
            >,
        > = __cordl_object.invoke("get_lightIdColorPairs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lightIdColorPairs(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::LightIdColorPair,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lightIdColorPairs", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MenuLightsPresetSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MenuLightsPresetSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
