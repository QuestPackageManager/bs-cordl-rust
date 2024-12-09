#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsIntPropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::BoundsInt,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::BoundsIntPropertyBag => "Unity.Properties.Internal"
    ."BoundsIntPropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::BoundsIntPropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::BoundsInt,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::BoundsIntPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag")]
impl crate::Unity::Properties::Internal::BoundsIntPropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+PositionProperty")]
    pub type PositionProperty = crate::Unity::Properties::Internal::BoundsIntPropertyBag_PositionProperty;
    #[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+SizeProperty")]
    pub type SizeProperty = crate::Unity::Properties::Internal::BoundsIntPropertyBag_SizeProperty;
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
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::BoundsIntPropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+PositionProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsIntPropertyBag_PositionProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::BoundsInt,
        crate::UnityEngine::Vector3Int,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+PositionProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::BoundsIntPropertyBag_PositionProperty =>
    "Unity.Properties.Internal"."BoundsIntPropertyBag/PositionProperty"
);
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+PositionProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::BoundsIntPropertyBag_PositionProperty {
    type Target = crate::Unity::Properties::Property_2<
        crate::UnityEngine::BoundsInt,
        crate::UnityEngine::Vector3Int,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+PositionProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::BoundsIntPropertyBag_PositionProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+PositionProperty")]
impl crate::Unity::Properties::Internal::BoundsIntPropertyBag_PositionProperty {
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
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+PositionProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::BoundsIntPropertyBag_PositionProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+SizeProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsIntPropertyBag_SizeProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::BoundsInt,
        crate::UnityEngine::Vector3Int,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+SizeProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::BoundsIntPropertyBag_SizeProperty =>
    "Unity.Properties.Internal"."BoundsIntPropertyBag/SizeProperty"
);
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+SizeProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::BoundsIntPropertyBag_SizeProperty {
    type Target = crate::Unity::Properties::Property_2<
        crate::UnityEngine::BoundsInt,
        crate::UnityEngine::Vector3Int,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+SizeProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::BoundsIntPropertyBag_SizeProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+SizeProperty")]
impl crate::Unity::Properties::Internal::BoundsIntPropertyBag_SizeProperty {
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
#[cfg(feature = "Unity+Properties+Internal+BoundsIntPropertyBag+SizeProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::BoundsIntPropertyBag_SizeProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
