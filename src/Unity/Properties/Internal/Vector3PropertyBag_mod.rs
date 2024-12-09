#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3PropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::Vector3PropertyBag
    => "Unity.Properties.Internal"."Vector3PropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::Vector3PropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Vector3,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::Vector3PropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag")]
impl crate::Unity::Properties::Internal::Vector3PropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+XProperty")]
    pub type XProperty = crate::Unity::Properties::Internal::Vector3PropertyBag_XProperty;
    #[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+YProperty")]
    pub type YProperty = crate::Unity::Properties::Internal::Vector3PropertyBag_YProperty;
    #[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+ZProperty")]
    pub type ZProperty = crate::Unity::Properties::Internal::Vector3PropertyBag_ZProperty;
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
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector3PropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+XProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3PropertyBag_XProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+XProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector3PropertyBag_XProperty =>
    "Unity.Properties.Internal"."Vector3PropertyBag/XProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+XProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector3PropertyBag_XProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector3, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+XProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector3PropertyBag_XProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+XProperty")]
impl crate::Unity::Properties::Internal::Vector3PropertyBag_XProperty {
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
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+XProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector3PropertyBag_XProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+YProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3PropertyBag_YProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+YProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector3PropertyBag_YProperty =>
    "Unity.Properties.Internal"."Vector3PropertyBag/YProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+YProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector3PropertyBag_YProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector3, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+YProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector3PropertyBag_YProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+YProperty")]
impl crate::Unity::Properties::Internal::Vector3PropertyBag_YProperty {
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
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+YProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector3PropertyBag_YProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+ZProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3PropertyBag_ZProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3,
        f32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+ZProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector3PropertyBag_ZProperty =>
    "Unity.Properties.Internal"."Vector3PropertyBag/ZProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+ZProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector3PropertyBag_ZProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Vector3, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+ZProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector3PropertyBag_ZProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+ZProperty")]
impl crate::Unity::Properties::Internal::Vector3PropertyBag_ZProperty {
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
#[cfg(feature = "Unity+Properties+Internal+Vector3PropertyBag+ZProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector3PropertyBag_ZProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
