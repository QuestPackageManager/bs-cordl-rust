#[cfg(feature = "UnityEngine+UIElements+VectorImage")]
#[repr(C)]
#[derive(Debug)]
pub struct VectorImage {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub version: i32,
    pub atlas: *mut crate::UnityEngine::Texture2D,
    pub vertices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::VectorImageVertex,
    >,
    pub indices: *mut quest_hook::libil2cpp::Il2CppArray<u16>,
    pub settings: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::GradientSettings,
    >,
    pub _cordl_size: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+UIElements+VectorImage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VectorImage =>
    "UnityEngine.UIElements"."VectorImage"
);
#[cfg(feature = "UnityEngine+UIElements+VectorImage")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VectorImage {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VectorImage")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VectorImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VectorImage")]
impl crate::UnityEngine::UIElements::VectorImage {
    pub fn get_height(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_height", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_width", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VectorImage")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::VectorImage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
