#[cfg(feature = "BakedLightTexturePacking")]
#[repr(C)]
#[derive(Debug)]
pub struct BakedLightTexturePacking {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BakedLightTexturePacking")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BakedLightTexturePacking {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BakedLightTexturePacking";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl std::ops::Deref for crate::GlobalNamespace::BakedLightTexturePacking {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl std::ops::DerefMut for crate::GlobalNamespace::BakedLightTexturePacking {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl crate::GlobalNamespace::BakedLightTexturePacking {
    pub const kBakedLightTexturePackingShaderName: &'static str = "Hidden/BakedLightTexturePacking";
    pub fn PackTextures(
        textures: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
            >,
        >,
        descriptor: crate::UnityEngine::RenderTextureDescriptor,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IReadOnlyList_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                                >,
                            >,
                            crate::UnityEngine::RenderTextureDescriptor,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        2usize,
                    >("PackTextures")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PackTextures", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = unsafe {
            cordl_method_info.invoke_unchecked((), (textures, descriptor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BakedLightTexturePacking {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
