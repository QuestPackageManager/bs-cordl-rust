#[cfg(feature = "BTSCharacter")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _characterName: *mut crate::System::String,
    pub _animator: *mut crate::UnityEngine::Animator,
    pub _btsCharacterMaterialSwitcher: *mut BTSCharacterMaterialSwitcher,
    pub _materialPropertyBlockController: *mut MaterialPropertyBlockController,
    pub _headTransform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "BTSCharacter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BTSCharacter => ""."BTSCharacter"
);
#[cfg(feature = "BTSCharacter")]
impl std::ops::Deref for BTSCharacter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacter")]
impl std::ops::DerefMut for BTSCharacter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacter")]
impl BTSCharacter {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetAlternativeAnimationAndMaterial(
        &mut self,
        animation: *mut crate::UnityEngine::AnimationClip,
        alternativeMaterialOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetAlternativeAnimationAndMaterial",
                (animation, alternativeMaterialOn),
            )?;
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
    pub fn get_animator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Animator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Animator = __cordl_object
            .invoke("get_animator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_characterName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_headTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_headTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_materialPropertyBlockController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MaterialPropertyBlockController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MaterialPropertyBlockController = __cordl_object
            .invoke("get_materialPropertyBlockController", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BTSCharacter")]
impl quest_hook::libil2cpp::ObjectType for BTSCharacter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
