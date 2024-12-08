#[cfg(feature = "UnityEngine+Rendering+ObjectIdRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectIdRequest {
    __cordl_parent: crate::System::Object,
    pub _destination_k__BackingField: *mut crate::UnityEngine::RenderTexture,
}
#[cfg(feature = "UnityEngine+Rendering+ObjectIdRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ObjectIdRequest =>
    "UnityEngine.Rendering"."ObjectIdRequest"
);
#[cfg(feature = "UnityEngine+Rendering+ObjectIdRequest")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ObjectIdRequest {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ObjectIdRequest")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ObjectIdRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ObjectIdRequest")]
impl crate::UnityEngine::Rendering::ObjectIdRequest {
    pub fn get_destination(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RenderTexture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RenderTexture = __cordl_object
            .invoke("get_destination", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Rendering+ObjectIdRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::ObjectIdRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
