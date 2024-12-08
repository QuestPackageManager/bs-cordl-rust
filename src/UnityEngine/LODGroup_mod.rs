#[cfg(feature = "UnityEngine+LODGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct LODGroup {
    __cordl_parent: crate::UnityEngine::Component,
}
#[cfg(feature = "UnityEngine+LODGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LODGroup => "UnityEngine"
    ."LODGroup"
);
#[cfg(feature = "UnityEngine+LODGroup")]
impl std::ops::Deref for crate::UnityEngine::LODGroup {
    type Target = crate::UnityEngine::Component;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LODGroup")]
impl std::ops::DerefMut for crate::UnityEngine::LODGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LODGroup")]
impl crate::UnityEngine::LODGroup {
    pub fn GetLODs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::LOD>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::LOD,
        > = __cordl_object.invoke("GetLODs", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecalculateBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateBounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLODs(
        &mut self,
        lods: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::LOD>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLODs", (lods))?;
        Ok(__cordl_ret)
    }
    pub fn get_lodCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lodCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_size(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_size", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+LODGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::LODGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
