#[cfg(feature = "UnityEngine+UIElements+UIR+VectorImageManager")]
#[repr(C)]
#[derive(Debug)]
pub struct VectorImageManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Atlas: *mut crate::UnityEngine::UIElements::AtlasBase,
    pub m_Registered: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::UIElements::VectorImage,
        *mut crate::UnityEngine::UIElements::UIR::VectorImageRenderInfo,
    >,
    pub m_RenderInfoPool: *mut crate::UnityEngine::UIElements::UIR::VectorImageRenderInfoPool,
    pub m_GradientRemapPool: *mut crate::UnityEngine::UIElements::UIR::GradientRemapPool,
    pub m_GradientSettingsAtlas: *mut crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas,
    pub m_LoggedExhaustedSettingsAtlas: bool,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+VectorImageManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::VectorImageManager
    => "UnityEngine.UIElements.UIR"."VectorImageManager"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+VectorImageManager")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::VectorImageManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+VectorImageManager")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::VectorImageManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+VectorImageManager")]
impl crate::UnityEngine::UIElements::UIR::VectorImageManager {
    pub fn AddUser(
        &mut self,
        vi: *mut crate::UnityEngine::UIElements::VectorImage,
        context: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::UIR::GradientRemap,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::UIR::GradientRemap = __cordl_object
            .invoke("AddUser", (vi, context))?;
        Ok(__cordl_ret)
    }
    pub fn Commit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Commit", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        atlas: *mut crate::UnityEngine::UIElements::AtlasBase,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (atlas))?;
        Ok(__cordl_object)
    }
    pub fn Register(
        &mut self,
        vi: *mut crate::UnityEngine::UIElements::VectorImage,
        context: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::UIR::VectorImageRenderInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::UIR::VectorImageRenderInfo = __cordl_object
            .invoke("Register", (vi, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        atlas: *mut crate::UnityEngine::UIElements::AtlasBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (atlas))?;
        Ok(__cordl_ret)
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("get_atlas", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+VectorImageManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::VectorImageManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
