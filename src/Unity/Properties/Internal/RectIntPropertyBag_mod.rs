#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct RectIntPropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::RectInt,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::RectIntPropertyBag
    => "Unity.Properties.Internal"."RectIntPropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::RectIntPropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::RectInt,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::RectIntPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag")]
impl crate::Unity::Properties::Internal::RectIntPropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+HeightProperty")]
    pub type HeightProperty = crate::Unity::Properties::Internal::RectIntPropertyBag_HeightProperty;
    #[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+WidthProperty")]
    pub type WidthProperty = crate::Unity::Properties::Internal::RectIntPropertyBag_WidthProperty;
    #[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+XProperty")]
    pub type XProperty = crate::Unity::Properties::Internal::RectIntPropertyBag_XProperty;
    #[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+YProperty")]
    pub type YProperty = crate::Unity::Properties::Internal::RectIntPropertyBag_YProperty;
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
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectIntPropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+HeightProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct RectIntPropertyBag_HeightProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::RectInt,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+HeightProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::RectIntPropertyBag_HeightProperty =>
    "Unity.Properties.Internal"."RectIntPropertyBag/HeightProperty"
);
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+HeightProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::RectIntPropertyBag_HeightProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::RectInt, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+HeightProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::RectIntPropertyBag_HeightProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+HeightProperty")]
impl crate::Unity::Properties::Internal::RectIntPropertyBag_HeightProperty {
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
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+HeightProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectIntPropertyBag_HeightProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+WidthProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct RectIntPropertyBag_WidthProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::RectInt,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+WidthProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::RectIntPropertyBag_WidthProperty =>
    "Unity.Properties.Internal"."RectIntPropertyBag/WidthProperty"
);
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+WidthProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::RectIntPropertyBag_WidthProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::RectInt, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+WidthProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::RectIntPropertyBag_WidthProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+WidthProperty")]
impl crate::Unity::Properties::Internal::RectIntPropertyBag_WidthProperty {
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
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+WidthProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectIntPropertyBag_WidthProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+XProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct RectIntPropertyBag_XProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::RectInt,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+XProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::RectIntPropertyBag_XProperty =>
    "Unity.Properties.Internal"."RectIntPropertyBag/XProperty"
);
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+XProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::RectIntPropertyBag_XProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::RectInt, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+XProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::RectIntPropertyBag_XProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+XProperty")]
impl crate::Unity::Properties::Internal::RectIntPropertyBag_XProperty {
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
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+XProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectIntPropertyBag_XProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+YProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct RectIntPropertyBag_YProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        crate::UnityEngine::RectInt,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+YProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::RectIntPropertyBag_YProperty =>
    "Unity.Properties.Internal"."RectIntPropertyBag/YProperty"
);
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+YProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::RectIntPropertyBag_YProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::RectInt, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+YProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::RectIntPropertyBag_YProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+YProperty")]
impl crate::Unity::Properties::Internal::RectIntPropertyBag_YProperty {
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
#[cfg(feature = "Unity+Properties+Internal+RectIntPropertyBag+YProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectIntPropertyBag_YProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
