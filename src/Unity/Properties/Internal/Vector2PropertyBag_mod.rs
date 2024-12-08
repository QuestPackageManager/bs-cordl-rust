#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector2PropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Vector2,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::Vector2PropertyBag
    => "Unity.Properties.Internal"."Vector2PropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::Vector2PropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Vector2,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::Vector2PropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag")]
impl crate::Unity::Properties::Internal::Vector2PropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+YProperty")]
    pub type YProperty = crate::Unity::Properties::Internal::Vector2PropertyBag_YProperty;
    #[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+XProperty")]
    pub type XProperty = crate::Unity::Properties::Internal::Vector2PropertyBag_XProperty;
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
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector2PropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+XProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector2PropertyBag_XProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector2,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+XProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector2PropertyBag_XProperty =>
    "Unity.Properties.Internal"."Vector2PropertyBag/XProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+XProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector2PropertyBag_XProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector2, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+XProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector2PropertyBag_XProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+XProperty")]
impl crate::Unity::Properties::Internal::Vector2PropertyBag_XProperty {
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
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+XProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector2PropertyBag_XProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+YProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector2PropertyBag_YProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector2,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+YProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector2PropertyBag_YProperty =>
    "Unity.Properties.Internal"."Vector2PropertyBag/YProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+YProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector2PropertyBag_YProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector2, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+YProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector2PropertyBag_YProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+YProperty")]
impl crate::Unity::Properties::Internal::Vector2PropertyBag_YProperty {
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
#[cfg(feature = "Unity+Properties+Internal+Vector2PropertyBag+YProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector2PropertyBag_YProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
