#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct LowerResBlitTexture {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::LowerResBlitTexture {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "LowerResBlitTexture";
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
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
impl std::ops::Deref for crate::UnityEngine::LowerResBlitTexture {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
impl std::ops::DerefMut for crate::UnityEngine::LowerResBlitTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
impl crate::UnityEngine::LowerResBlitTexture {
    pub fn LowerResBlitTextureDontStripMe(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::LowerResBlitTexture as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("LowerResBlitTextureDontStripMe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::LowerResBlitTexture as
                    quest_hook::libil2cpp::Type > ::class(),
                    "LowerResBlitTextureDontStripMe", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::LowerResBlitTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
