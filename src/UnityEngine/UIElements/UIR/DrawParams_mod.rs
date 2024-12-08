#[cfg(feature = "UnityEngine+UIElements+UIR+DrawParams")]
#[repr(C)]
#[derive(Debug)]
pub struct DrawParams {
    __cordl_parent: crate::System::Object,
    pub view: *mut crate::System::Collections::Generic::Stack_1<
        crate::UnityEngine::Matrix4x4,
    >,
    pub scissor: *mut crate::System::Collections::Generic::Stack_1<
        crate::UnityEngine::Rect,
    >,
    pub renderTexture: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::RenderTexture,
    >,
    pub defaultMaterial: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Material,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DrawParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::DrawParams =>
    "UnityEngine.UIElements.UIR"."DrawParams"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+DrawParams")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::DrawParams {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DrawParams")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::DrawParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DrawParams")]
impl crate::UnityEngine::UIElements::UIR::DrawParams {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+UIR+DrawParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::DrawParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
