#[cfg(feature = "ConditionalMaterialSwitcher")]
#[repr(C)]
#[derive(Debug)]
pub struct ConditionalMaterialSwitcher {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _material0: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _material1: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BoolSO>,
    pub _renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
}
#[cfg(feature = "ConditionalMaterialSwitcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ConditionalMaterialSwitcher =>
    ""."ConditionalMaterialSwitcher"
);
#[cfg(feature = "ConditionalMaterialSwitcher")]
impl std::ops::Deref for crate::GlobalNamespace::ConditionalMaterialSwitcher {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConditionalMaterialSwitcher")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConditionalMaterialSwitcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConditionalMaterialSwitcher")]
impl crate::GlobalNamespace::ConditionalMaterialSwitcher {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "ConditionalMaterialSwitcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConditionalMaterialSwitcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
