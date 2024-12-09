#[cfg(feature = "TMPro+TMP_MaterialManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_MaterialManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_MaterialManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_MaterialManager => "TMPro"
    ."TMP_MaterialManager"
);
#[cfg(feature = "TMPro+TMP_MaterialManager")]
impl std::ops::Deref for crate::TMPro::TMP_MaterialManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager")]
impl std::ops::DerefMut for crate::TMPro::TMP_MaterialManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager")]
impl crate::TMPro::TMP_MaterialManager {
    #[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
    pub type FallbackMaterial = crate::TMPro::TMP_MaterialManager_FallbackMaterial;
    #[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
    pub type MaskingMaterial = crate::TMPro::TMP_MaterialManager_MaskingMaterial;
    #[cfg(feature = "TMPro+TMP_MaterialManager+__c__DisplayClass11_0")]
    pub type __c__DisplayClass11_0 = crate::TMPro::TMP_MaterialManager___c__DisplayClass11_0;
    #[cfg(feature = "TMPro+TMP_MaterialManager+__c__DisplayClass12_0")]
    pub type __c__DisplayClass12_0 = crate::TMPro::TMP_MaterialManager___c__DisplayClass12_0;
    #[cfg(feature = "TMPro+TMP_MaterialManager+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::TMPro::TMP_MaterialManager___c__DisplayClass13_0;
    #[cfg(feature = "TMPro+TMP_MaterialManager+__c__DisplayClass9_0")]
    pub type __c__DisplayClass9_0 = crate::TMPro::TMP_MaterialManager___c__DisplayClass9_0;
}
#[cfg(feature = "TMPro+TMP_MaterialManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_MaterialManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_MaterialManager_FallbackMaterial {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub fallbackID: i64,
    pub sourceMaterial: *mut crate::UnityEngine::Material,
    pub sourceMaterialCRC: i32,
    pub fallbackMaterial: *mut crate::UnityEngine::Material,
    pub count: i32,
}
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_MaterialManager_FallbackMaterial =>
    "TMPro"."TMP_MaterialManager/FallbackMaterial"
);
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
impl std::ops::Deref for crate::TMPro::TMP_MaterialManager_FallbackMaterial {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
impl std::ops::DerefMut for crate::TMPro::TMP_MaterialManager_FallbackMaterial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
impl crate::TMPro::TMP_MaterialManager_FallbackMaterial {
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
}
#[cfg(feature = "TMPro+TMP_MaterialManager+FallbackMaterial")]
impl quest_hook::libil2cpp::ObjectType
for crate::TMPro::TMP_MaterialManager_FallbackMaterial {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_MaterialManager_MaskingMaterial {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub baseMaterial: *mut crate::UnityEngine::Material,
    pub stencilMaterial: *mut crate::UnityEngine::Material,
    pub count: i32,
    pub stencilID: i32,
}
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_MaterialManager_MaskingMaterial =>
    "TMPro"."TMP_MaterialManager/MaskingMaterial"
);
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
impl std::ops::Deref for crate::TMPro::TMP_MaterialManager_MaskingMaterial {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
impl std::ops::DerefMut for crate::TMPro::TMP_MaterialManager_MaskingMaterial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
impl crate::TMPro::TMP_MaterialManager_MaskingMaterial {
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
}
#[cfg(feature = "TMPro+TMP_MaterialManager+MaskingMaterial")]
impl quest_hook::libil2cpp::ObjectType
for crate::TMPro::TMP_MaterialManager_MaskingMaterial {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
