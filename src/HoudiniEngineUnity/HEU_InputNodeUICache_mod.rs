#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputAssetUICache")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputNodeUICache_HEU_InputAssetUICache {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputAssetUICache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputAssetUICache =>
    "HoudiniEngineUnity"."HEU_InputNodeUICache/HEU_InputAssetUICache"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputAssetUICache")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputAssetUICache {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputAssetUICache")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputAssetUICache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputAssetUICache")]
impl crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputAssetUICache {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputAssetUICache")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputAssetUICache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputNodeUICache {
    __cordl_parent: crate::System::Object,
    pub _inputObjectCache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputObjectUICache,
    >,
    pub _inputAssetCache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputAssetUICache,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputNodeUICache =>
    "HoudiniEngineUnity"."HEU_InputNodeUICache"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputNodeUICache {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputNodeUICache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache")]
impl crate::HoudiniEngineUnity::HEU_InputNodeUICache {
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputObjectUICache")]
    pub type HEU_InputObjectUICache = crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputObjectUICache;
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputAssetUICache")]
    pub type HEU_InputAssetUICache = crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputAssetUICache;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputNodeUICache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputObjectUICache")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputNodeUICache_HEU_InputObjectUICache {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputObjectUICache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputObjectUICache =>
    "HoudiniEngineUnity"."HEU_InputNodeUICache/HEU_InputObjectUICache"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputObjectUICache")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputObjectUICache {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputObjectUICache")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputObjectUICache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputObjectUICache")]
impl crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputObjectUICache {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNodeUICache+HEU_InputObjectUICache")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputNodeUICache_HEU_InputObjectUICache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
