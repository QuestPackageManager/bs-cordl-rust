#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3IntPropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Vector3Int,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector3IntPropertyBag => "Unity.Properties.Internal"
    ."Vector3IntPropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::Vector3IntPropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Vector3Int,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::Vector3IntPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag")]
impl crate::Unity::Properties::Internal::Vector3IntPropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+XProperty")]
    pub type XProperty = crate::Unity::Properties::Internal::Vector3IntPropertyBag_XProperty;
    #[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+YProperty")]
    pub type YProperty = crate::Unity::Properties::Internal::Vector3IntPropertyBag_YProperty;
    #[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+ZProperty")]
    pub type ZProperty = crate::Unity::Properties::Internal::Vector3IntPropertyBag_ZProperty;
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
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector3IntPropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+XProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3IntPropertyBag_XProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3Int,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+XProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector3IntPropertyBag_XProperty =>
    "Unity.Properties.Internal"."Vector3IntPropertyBag/XProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+XProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_XProperty {
    type Target = crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3Int,
        i32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+XProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_XProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+XProperty")]
impl crate::Unity::Properties::Internal::Vector3IntPropertyBag_XProperty {
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
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+XProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_XProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+YProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3IntPropertyBag_YProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3Int,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+YProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector3IntPropertyBag_YProperty =>
    "Unity.Properties.Internal"."Vector3IntPropertyBag/YProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+YProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_YProperty {
    type Target = crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3Int,
        i32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+YProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_YProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+YProperty")]
impl crate::Unity::Properties::Internal::Vector3IntPropertyBag_YProperty {
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
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+YProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_YProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+ZProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3IntPropertyBag_ZProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3Int,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+ZProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::Vector3IntPropertyBag_ZProperty =>
    "Unity.Properties.Internal"."Vector3IntPropertyBag/ZProperty"
);
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+ZProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_ZProperty {
    type Target = crate::Unity::Properties::Property_2<
        crate::UnityEngine::Vector3Int,
        i32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+ZProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_ZProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+ZProperty")]
impl crate::Unity::Properties::Internal::Vector3IntPropertyBag_ZProperty {
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
#[cfg(feature = "Unity+Properties+Internal+Vector3IntPropertyBag+ZProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::Vector3IntPropertyBag_ZProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
