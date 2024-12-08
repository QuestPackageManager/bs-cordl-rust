#[cfg(feature = "BloomPrePassBackgroundGradient")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundGradient {
    __cordl_parent: BloomPrePassBackgroundTextureGradient,
    pub _gradient: *mut crate::UnityEngine::Gradient,
}
#[cfg(feature = "BloomPrePassBackgroundGradient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomPrePassBackgroundGradient => ""
    ."BloomPrePassBackgroundGradient"
);
#[cfg(feature = "BloomPrePassBackgroundGradient")]
impl std::ops::Deref for BloomPrePassBackgroundGradient {
    type Target = BloomPrePassBackgroundTextureGradient;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundGradient")]
impl std::ops::DerefMut for BloomPrePassBackgroundGradient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundGradient")]
impl BloomPrePassBackgroundGradient {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UpdatePixels(
        &mut self,
        pixels: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Color32>,
        numberOfPixels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePixels", (pixels, numberOfPixels))?;
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
#[cfg(feature = "BloomPrePassBackgroundGradient")]
impl quest_hook::libil2cpp::ObjectType for BloomPrePassBackgroundGradient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}