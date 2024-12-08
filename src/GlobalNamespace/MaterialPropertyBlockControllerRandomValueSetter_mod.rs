#[cfg(feature = "MaterialPropertyBlockControllerRandomValueSetter")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyBlockControllerRandomValueSetter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _materialPropertyBlockController: *mut MaterialPropertyBlockController,
    pub _propertyName: *mut crate::System::String,
    pub _min: f32,
    pub _max: f32,
    pub _materialPropertyBlocks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::MaterialPropertyBlock,
    >,
    pub _propertyId: i32,
}
#[cfg(feature = "MaterialPropertyBlockControllerRandomValueSetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MaterialPropertyBlockControllerRandomValueSetter => ""
    ."MaterialPropertyBlockControllerRandomValueSetter"
);
#[cfg(feature = "MaterialPropertyBlockControllerRandomValueSetter")]
impl std::ops::Deref for MaterialPropertyBlockControllerRandomValueSetter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyBlockControllerRandomValueSetter")]
impl std::ops::DerefMut for MaterialPropertyBlockControllerRandomValueSetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyBlockControllerRandomValueSetter")]
impl MaterialPropertyBlockControllerRandomValueSetter {
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshPropertyId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshPropertyId", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplyParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyParams", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MaterialPropertyBlockControllerRandomValueSetter")]
impl quest_hook::libil2cpp::ObjectType
for MaterialPropertyBlockControllerRandomValueSetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
