#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector4PropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Vector4,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::Vector4PropertyBag
    => "Unity.Properties.Internal"."Vector4PropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::Vector4PropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Vector4,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::Vector4PropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag")]
impl crate::Unity::Properties::Internal::Vector4PropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+XProperty")]
    pub type XProperty = crate::Unity::Properties::Internal::Vector4PropertyBag_XProperty;
    #[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+YProperty")]
    pub type YProperty = crate::Unity::Properties::Internal::Vector4PropertyBag_YProperty;
    #[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+ZProperty")]
    pub type ZProperty = crate::Unity::Properties::Internal::Vector4PropertyBag_ZProperty;
    #[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+WProperty")]
    pub type WProperty = crate::Unity::Properties::Internal::Vector4PropertyBag_WProperty;
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
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector4PropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+WProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector4PropertyBag_WProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector4,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+WProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector4PropertyBag_WProperty =>
    "Unity.Properties.Internal"."Vector4PropertyBag/WProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+WProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector4PropertyBag_WProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector4, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+WProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector4PropertyBag_WProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+WProperty")]
impl crate::Unity::Properties::Internal::Vector4PropertyBag_WProperty {
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+WProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector4PropertyBag_WProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+XProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector4PropertyBag_XProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector4,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+XProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector4PropertyBag_XProperty =>
    "Unity.Properties.Internal"."Vector4PropertyBag/XProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+XProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector4PropertyBag_XProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector4, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+XProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector4PropertyBag_XProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+XProperty")]
impl crate::Unity::Properties::Internal::Vector4PropertyBag_XProperty {
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+XProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector4PropertyBag_XProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+YProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector4PropertyBag_YProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector4,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+YProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector4PropertyBag_YProperty =>
    "Unity.Properties.Internal"."Vector4PropertyBag/YProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+YProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector4PropertyBag_YProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector4, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+YProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector4PropertyBag_YProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+YProperty")]
impl crate::Unity::Properties::Internal::Vector4PropertyBag_YProperty {
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+YProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector4PropertyBag_YProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+ZProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector4PropertyBag_ZProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector4,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+ZProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector4PropertyBag_ZProperty =>
    "Unity.Properties.Internal"."Vector4PropertyBag/ZProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+ZProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector4PropertyBag_ZProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector4, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+ZProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector4PropertyBag_ZProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+ZProperty")]
impl crate::Unity::Properties::Internal::Vector4PropertyBag_ZProperty {
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector4PropertyBag+ZProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector4PropertyBag_ZProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
