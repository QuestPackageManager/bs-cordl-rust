#[cfg(feature = "MaterialPropertyBlockControllerArrayRandomValueSetter")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyBlockControllerArrayRandomValueSetter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _materialPropertyBlockControllers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MaterialPropertyBlockController,
        >,
    >,
    pub _propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _min: crate::UnityEngine::Vector3,
    pub _max: crate::UnityEngine::Vector3,
    pub _materialPropertyBlocks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::MaterialPropertyBlock,
        >,
    >,
    pub _propertyId: i32,
}
#[cfg(feature = "MaterialPropertyBlockControllerArrayRandomValueSetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MaterialPropertyBlockControllerArrayRandomValueSetter => ""
    ."MaterialPropertyBlockControllerArrayRandomValueSetter"
);
#[cfg(feature = "MaterialPropertyBlockControllerArrayRandomValueSetter")]
impl std::ops::Deref
for crate::GlobalNamespace::MaterialPropertyBlockControllerArrayRandomValueSetter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyBlockControllerArrayRandomValueSetter")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MaterialPropertyBlockControllerArrayRandomValueSetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyBlockControllerArrayRandomValueSetter")]
impl crate::GlobalNamespace::MaterialPropertyBlockControllerArrayRandomValueSetter {
    pub fn ApplyParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshPropertyId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshPropertyId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "MaterialPropertyBlockControllerArrayRandomValueSetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyBlockControllerArrayRandomValueSetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
