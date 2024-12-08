#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Noise3DTexturesGenerator_MaterialPropertyNameCouple {
    pub texturePropertyName: *mut crate::System::String,
    pub material: *mut crate::UnityEngine::Material,
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple => ""
    ."Noise3DTexturesGenerator/MaterialPropertyNameCouple"
);
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
impl crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Noise3DTexturesGenerator_MaterialTextureParamsCouple {
    pub globalPropertyName: *mut crate::System::String,
    pub materialPropertyNameCouples: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple,
    >,
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple => ""
    ."Noise3DTexturesGenerator/MaterialTextureParamsCouple"
);
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
impl crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {}
#[cfg(feature = "Noise3DTexturesGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct Noise3DTexturesGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _data: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple,
    >,
}
#[cfg(feature = "Noise3DTexturesGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for Noise3DTexturesGenerator => ""
    ."Noise3DTexturesGenerator"
);
#[cfg(feature = "Noise3DTexturesGenerator")]
impl std::ops::Deref for Noise3DTexturesGenerator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Noise3DTexturesGenerator")]
impl std::ops::DerefMut for Noise3DTexturesGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Noise3DTexturesGenerator")]
impl Noise3DTexturesGenerator {
    #[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
    pub type MaterialTextureParamsCouple = crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple;
    #[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
    pub type MaterialPropertyNameCouple = crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple;
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
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
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
#[cfg(feature = "Noise3DTexturesGenerator")]
impl quest_hook::libil2cpp::ObjectType for Noise3DTexturesGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
