#[cfg(feature = "FireEffectShaderHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct FireEffectShaderHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "FireEffectShaderHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FireEffectShaderHelper => ""."FireEffectShaderHelper"
);
#[cfg(feature = "FireEffectShaderHelper")]
impl std::ops::Deref for FireEffectShaderHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FireEffectShaderHelper")]
impl std::ops::DerefMut for FireEffectShaderHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FireEffectShaderHelper")]
impl FireEffectShaderHelper {}
#[cfg(feature = "FireEffectShaderHelper")]
impl quest_hook::libil2cpp::ObjectType for FireEffectShaderHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
