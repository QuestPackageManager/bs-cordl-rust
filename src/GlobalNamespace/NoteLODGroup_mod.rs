#[cfg(feature = "NoteLODGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteLODGroup {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CustomLODGroup>,
    pub _postProcessEnabled: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BoolSO>,
}
#[cfg(feature = "NoteLODGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteLODGroup => ""
    ."NoteLODGroup"
);
#[cfg(feature = "NoteLODGroup")]
impl std::ops::Deref for crate::GlobalNamespace::NoteLODGroup {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CustomLODGroup>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteLODGroup")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteLODGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteLODGroup")]
impl crate::GlobalNamespace::NoteLODGroup {
    pub fn GetStartupLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetStartupLevel", ())?;
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
    pub fn get_meshFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter> = __cordl_object
            .invoke("get_meshFilter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_meshRenderer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer> = __cordl_object
            .invoke("get_meshRenderer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteLODGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteLODGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
