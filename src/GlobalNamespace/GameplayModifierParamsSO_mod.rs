#[cfg(feature = "GameplayModifierParamsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifierParamsSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _modifierNameLocalizationKey: *mut crate::System::String,
    pub _descriptionLocalizationKey: *mut crate::System::String,
    pub _multiplier: f32,
    pub _multiplierConditionallyValid: bool,
    pub _icon: *mut crate::UnityEngine::Sprite,
    pub _mutuallyExclusives: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _requires: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _requiredBy: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _isInBeta: bool,
}
#[cfg(feature = "GameplayModifierParamsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifierParamsSO => ""
    ."GameplayModifierParamsSO"
);
#[cfg(feature = "GameplayModifierParamsSO")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifierParamsSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierParamsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifierParamsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierParamsSO")]
impl crate::GlobalNamespace::GameplayModifierParamsSO {
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
    pub fn get_isInBeta(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInBeta", ())?;
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
    pub fn get_multiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_multiplier", ())?;
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
    pub fn get_mutuallyExclusives(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::GameplayModifierParamsSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::GameplayModifierParamsSO,
        > = __cordl_object.invoke("get_mutuallyExclusives", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_requiredBy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::GameplayModifierParamsSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::GameplayModifierParamsSO,
        > = __cordl_object.invoke("get_requiredBy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_requires(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::GameplayModifierParamsSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::GameplayModifierParamsSO,
        > = __cordl_object.invoke("get_requires", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameplayModifierParamsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifierParamsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
