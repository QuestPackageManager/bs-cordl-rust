#[cfg(feature = "HEU_BoundingVolume")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_BoundingVolume {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "HEU_BoundingVolume")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HEU_BoundingVolume => ""."HEU_BoundingVolume"
);
#[cfg(feature = "HEU_BoundingVolume")]
impl std::ops::Deref for HEU_BoundingVolume {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_BoundingVolume")]
impl std::ops::DerefMut for HEU_BoundingVolume {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_BoundingVolume")]
impl HEU_BoundingVolume {
    pub fn GetAllIntersectingObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object.invoke("GetAllIntersectingObjects", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn get_BoundingCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Collider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Collider = __cordl_object
            .invoke("get_BoundingCollider", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HEU_BoundingVolume")]
impl quest_hook::libil2cpp::ObjectType for HEU_BoundingVolume {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}