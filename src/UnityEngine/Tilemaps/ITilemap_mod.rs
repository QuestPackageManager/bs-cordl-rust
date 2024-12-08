#[cfg(feature = "UnityEngine+Tilemaps+ITilemap")]
#[repr(C)]
#[derive(Debug)]
pub struct ITilemap {
    __cordl_parent: crate::System::Object,
    pub m_Tilemap: *mut crate::UnityEngine::Tilemaps::Tilemap,
    pub m_AddToList: bool,
    pub m_RefreshCount: i32,
    pub m_RefreshPos: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Vector3Int,
    >,
}
#[cfg(feature = "UnityEngine+Tilemaps+ITilemap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::ITilemap =>
    "UnityEngine.Tilemaps"."ITilemap"
);
#[cfg(feature = "UnityEngine+Tilemaps+ITilemap")]
impl std::ops::Deref for crate::UnityEngine::Tilemaps::ITilemap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+ITilemap")]
impl std::ops::DerefMut for crate::UnityEngine::Tilemaps::ITilemap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+ITilemap")]
impl crate::UnityEngine::Tilemaps::ITilemap {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RefreshTile(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshTile", (position))?;
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
}
#[cfg(feature = "UnityEngine+Tilemaps+ITilemap")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Tilemaps::ITilemap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
