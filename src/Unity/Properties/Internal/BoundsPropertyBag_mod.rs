#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsPropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Bounds,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::BoundsPropertyBag
    => "Unity.Properties.Internal"."BoundsPropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::BoundsPropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Bounds,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::BoundsPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag")]
impl crate::Unity::Properties::Internal::BoundsPropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+CenterProperty")]
    pub type CenterProperty = crate::Unity::Properties::Internal::BoundsPropertyBag_CenterProperty;
    #[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+ExtentsProperty")]
    pub type ExtentsProperty = crate::Unity::Properties::Internal::BoundsPropertyBag_ExtentsProperty;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::BoundsPropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+CenterProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsPropertyBag_CenterProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Bounds,
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+CenterProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::BoundsPropertyBag_CenterProperty =>
    "Unity.Properties.Internal"."BoundsPropertyBag/CenterProperty"
);
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+CenterProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::BoundsPropertyBag_CenterProperty {
    type Target = crate::Unity::Properties::Property_2<
        crate::UnityEngine::Bounds,
        crate::UnityEngine::Vector3,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+CenterProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::BoundsPropertyBag_CenterProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+CenterProperty")]
impl crate::Unity::Properties::Internal::BoundsPropertyBag_CenterProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+CenterProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::BoundsPropertyBag_CenterProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+ExtentsProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsPropertyBag_ExtentsProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Bounds,
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+ExtentsProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::BoundsPropertyBag_ExtentsProperty =>
    "Unity.Properties.Internal"."BoundsPropertyBag/ExtentsProperty"
);
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+ExtentsProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::BoundsPropertyBag_ExtentsProperty {
    type Target = crate::Unity::Properties::Property_2<
        crate::UnityEngine::Bounds,
        crate::UnityEngine::Vector3,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+ExtentsProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::BoundsPropertyBag_ExtentsProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+ExtentsProperty")]
impl crate::Unity::Properties::Internal::BoundsPropertyBag_ExtentsProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+Internal+BoundsPropertyBag+ExtentsProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::BoundsPropertyBag_ExtentsProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
