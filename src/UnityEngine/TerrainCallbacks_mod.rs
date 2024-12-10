#[cfg(feature = "UnityEngine+TerrainCallbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainCallbacks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TerrainCallbacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainCallbacks => "UnityEngine"
    ."TerrainCallbacks"
);
#[cfg(feature = "UnityEngine+TerrainCallbacks")]
impl std::ops::Deref for crate::UnityEngine::TerrainCallbacks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainCallbacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks")]
impl crate::UnityEngine::TerrainCallbacks {
    #[cfg(feature = "UnityEngine+TerrainCallbacks+HeightmapChangedCallback")]
    pub type HeightmapChangedCallback = crate::UnityEngine::TerrainCallbacks_HeightmapChangedCallback;
    #[cfg(feature = "UnityEngine+TerrainCallbacks+TextureChangedCallback")]
    pub type TextureChangedCallback = crate::UnityEngine::TerrainCallbacks_TextureChangedCallback;
}
#[cfg(feature = "UnityEngine+TerrainCallbacks")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TerrainCallbacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+HeightmapChangedCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainCallbacks_HeightmapChangedCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+HeightmapChangedCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TerrainCallbacks_HeightmapChangedCallback => "UnityEngine"
    ."TerrainCallbacks/HeightmapChangedCallback"
);
#[cfg(feature = "UnityEngine+TerrainCallbacks+HeightmapChangedCallback")]
impl std::ops::Deref for crate::UnityEngine::TerrainCallbacks_HeightmapChangedCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+HeightmapChangedCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::TerrainCallbacks_HeightmapChangedCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+HeightmapChangedCallback")]
impl crate::UnityEngine::TerrainCallbacks_HeightmapChangedCallback {
    pub fn Invoke(
        &mut self,
        terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
        heightRegion: crate::UnityEngine::RectInt,
        synched: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (terrain, heightRegion, synched))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+HeightmapChangedCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TerrainCallbacks_HeightmapChangedCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+TextureChangedCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainCallbacks_TextureChangedCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+TextureChangedCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TerrainCallbacks_TextureChangedCallback => "UnityEngine"
    ."TerrainCallbacks/TextureChangedCallback"
);
#[cfg(feature = "UnityEngine+TerrainCallbacks+TextureChangedCallback")]
impl std::ops::Deref for crate::UnityEngine::TerrainCallbacks_TextureChangedCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+TextureChangedCallback")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainCallbacks_TextureChangedCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+TextureChangedCallback")]
impl crate::UnityEngine::TerrainCallbacks_TextureChangedCallback {
    pub fn Invoke(
        &mut self,
        terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
        textureName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        texelRegion: crate::UnityEngine::RectInt,
        synched: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (terrain, textureName, texelRegion, synched))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TerrainCallbacks+TextureChangedCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TerrainCallbacks_TextureChangedCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
