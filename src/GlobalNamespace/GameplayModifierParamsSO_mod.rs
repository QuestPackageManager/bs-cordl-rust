#[cfg(feature = "GameplayModifierParamsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifierParamsSO {
    __cordl_parent: PersistentScriptableObject,
    pub _modifierNameLocalizationKey: *mut crate::System::String,
    pub _descriptionLocalizationKey: *mut crate::System::String,
    pub _multiplier: f32,
    pub _multiplierConditionallyValid: bool,
    pub _icon: *mut crate::UnityEngine::Sprite,
    pub _mutuallyExclusives: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut GameplayModifierParamsSO,
    >,
    pub _requires: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut GameplayModifierParamsSO,
    >,
    pub _requiredBy: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut GameplayModifierParamsSO,
    >,
    pub _isInBeta: bool,
}
#[cfg(feature = "GameplayModifierParamsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameplayModifierParamsSO => ""
    ."GameplayModifierParamsSO"
);
#[cfg(feature = "GameplayModifierParamsSO")]
impl std::ops::Deref for GameplayModifierParamsSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierParamsSO")]
impl std::ops::DerefMut for GameplayModifierParamsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierParamsSO")]
impl GameplayModifierParamsSO {
    pub fn get_multiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_multiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInBeta(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInBeta", ())?;
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
    pub fn get_descriptionLocalizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_descriptionLocalizationKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplierConditionallyValid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_multiplierConditionallyValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_requiredBy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut GameplayModifierParamsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut GameplayModifierParamsSO,
        > = __cordl_object.invoke("get_requiredBy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_modifierNameLocalizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_modifierNameLocalizationKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mutuallyExclusives(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut GameplayModifierParamsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut GameplayModifierParamsSO,
        > = __cordl_object.invoke("get_mutuallyExclusives", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_requires(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut GameplayModifierParamsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut GameplayModifierParamsSO,
        > = __cordl_object.invoke("get_requires", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_icon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_icon", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GameplayModifierParamsSO")]
impl quest_hook::libil2cpp::ObjectType for GameplayModifierParamsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
