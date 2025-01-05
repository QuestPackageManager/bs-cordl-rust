#[cfg(feature = "MaterialSwapTransitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialSwapTransitionSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BaseTransitionSO>,
    pub _normalMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _highlightedMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _pressedMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _disabledMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _selectedMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _selectedAndHighlightedMaterial: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
}
#[cfg(feature = "MaterialSwapTransitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MaterialSwapTransitionSO => ""
    ."MaterialSwapTransitionSO"
);
#[cfg(feature = "MaterialSwapTransitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::MaterialSwapTransitionSO {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BaseTransitionSO>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialSwapTransitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MaterialSwapTransitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialSwapTransitionSO")]
impl crate::GlobalNamespace::MaterialSwapTransitionSO {
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
    pub fn get_disabledMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_disabledMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlightedMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_highlightedMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_normalMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_pressedMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedAndHighlightedMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_selectedAndHighlightedMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_selectedMaterial", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MaterialSwapTransitionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialSwapTransitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
