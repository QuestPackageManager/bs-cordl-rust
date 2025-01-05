#[cfg(feature = "UnityEngine+ColorUsageAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorUsageAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>,
    pub showAlpha: bool,
    pub hdr: bool,
    pub minBrightness: f32,
    pub maxBrightness: f32,
    pub minExposureValue: f32,
    pub maxExposureValue: f32,
}
#[cfg(feature = "UnityEngine+ColorUsageAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ColorUsageAttribute =>
    "UnityEngine"."ColorUsageAttribute"
);
#[cfg(feature = "UnityEngine+ColorUsageAttribute")]
impl std::ops::Deref for crate::UnityEngine::ColorUsageAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ColorUsageAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::ColorUsageAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ColorUsageAttribute")]
impl crate::UnityEngine::ColorUsageAttribute {
    pub fn New__cordl_bool0(
        showAlpha: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (showAlpha))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        showAlpha: bool,
        hdr: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (showAlpha, hdr))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        showAlpha: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (showAlpha))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        showAlpha: bool,
        hdr: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (showAlpha, hdr))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ColorUsageAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ColorUsageAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
