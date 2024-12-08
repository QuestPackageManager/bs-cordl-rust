#[cfg(feature = "UnityEngine+AvatarMask")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarMask {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+AvatarMask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AvatarMask => "UnityEngine"
    ."AvatarMask"
);
#[cfg(feature = "UnityEngine+AvatarMask")]
impl std::ops::Deref for crate::UnityEngine::AvatarMask {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AvatarMask")]
impl std::ops::DerefMut for crate::UnityEngine::AvatarMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AvatarMask")]
impl crate::UnityEngine::AvatarMask {
    pub fn GetHumanoidBodyPartActive(
        &mut self,
        index: crate::UnityEngine::AvatarMaskBodyPart,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetHumanoidBodyPartActive", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetTransformActive(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetTransformActive", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetTransformPath(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTransformPath", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetTransformWeight(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetTransformWeight", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_transformCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_transformCount", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AvatarMask")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AvatarMask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
