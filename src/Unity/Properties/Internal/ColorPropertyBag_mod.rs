#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorPropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Color,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::ColorPropertyBag =>
    "Unity.Properties.Internal"."ColorPropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::ColorPropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Color,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::ColorPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag")]
impl crate::Unity::Properties::Internal::ColorPropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+AProperty")]
    pub type AProperty = crate::Unity::Properties::Internal::ColorPropertyBag_AProperty;
    #[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+BProperty")]
    pub type BProperty = crate::Unity::Properties::Internal::ColorPropertyBag_BProperty;
    #[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+GProperty")]
    pub type GProperty = crate::Unity::Properties::Internal::ColorPropertyBag_GProperty;
    #[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+RProperty")]
    pub type RProperty = crate::Unity::Properties::Internal::ColorPropertyBag_RProperty;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::ColorPropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+AProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorPropertyBag_AProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<crate::UnityEngine::Color, f32>,
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+AProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::ColorPropertyBag_AProperty =>
    "Unity.Properties.Internal"."ColorPropertyBag/AProperty"
);
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+AProperty")]
impl std::ops::Deref for crate::Unity::Properties::Internal::ColorPropertyBag_AProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Color, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+AProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::ColorPropertyBag_AProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+AProperty")]
impl crate::Unity::Properties::Internal::ColorPropertyBag_AProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+AProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::ColorPropertyBag_AProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+BProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorPropertyBag_BProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<crate::UnityEngine::Color, f32>,
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+BProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::ColorPropertyBag_BProperty =>
    "Unity.Properties.Internal"."ColorPropertyBag/BProperty"
);
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+BProperty")]
impl std::ops::Deref for crate::Unity::Properties::Internal::ColorPropertyBag_BProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Color, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+BProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::ColorPropertyBag_BProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+BProperty")]
impl crate::Unity::Properties::Internal::ColorPropertyBag_BProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+BProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::ColorPropertyBag_BProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+GProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorPropertyBag_GProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<crate::UnityEngine::Color, f32>,
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+GProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::ColorPropertyBag_GProperty =>
    "Unity.Properties.Internal"."ColorPropertyBag/GProperty"
);
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+GProperty")]
impl std::ops::Deref for crate::Unity::Properties::Internal::ColorPropertyBag_GProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Color, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+GProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::ColorPropertyBag_GProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+GProperty")]
impl crate::Unity::Properties::Internal::ColorPropertyBag_GProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+GProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::ColorPropertyBag_GProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+RProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorPropertyBag_RProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<crate::UnityEngine::Color, f32>,
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+RProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::ColorPropertyBag_RProperty =>
    "Unity.Properties.Internal"."ColorPropertyBag/RProperty"
);
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+RProperty")]
impl std::ops::Deref for crate::Unity::Properties::Internal::ColorPropertyBag_RProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Color, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+RProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::ColorPropertyBag_RProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+RProperty")]
impl crate::Unity::Properties::Internal::ColorPropertyBag_RProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+ColorPropertyBag+RProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::ColorPropertyBag_RProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
