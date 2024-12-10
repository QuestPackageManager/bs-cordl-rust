#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct RectPropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Rect,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::RectPropertyBag =>
    "Unity.Properties.Internal"."RectPropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::RectPropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        crate::UnityEngine::Rect,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::RectPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag")]
impl crate::Unity::Properties::Internal::RectPropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+HeightProperty")]
    pub type HeightProperty = crate::Unity::Properties::Internal::RectPropertyBag_HeightProperty;
    #[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+WidthProperty")]
    pub type WidthProperty = crate::Unity::Properties::Internal::RectPropertyBag_WidthProperty;
    #[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+XProperty")]
    pub type XProperty = crate::Unity::Properties::Internal::RectPropertyBag_XProperty;
    #[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+YProperty")]
    pub type YProperty = crate::Unity::Properties::Internal::RectPropertyBag_YProperty;
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
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectPropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+HeightProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct RectPropertyBag_HeightProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<crate::UnityEngine::Rect, f32>,
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+HeightProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::RectPropertyBag_HeightProperty =>
    "Unity.Properties.Internal"."RectPropertyBag/HeightProperty"
);
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+HeightProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::RectPropertyBag_HeightProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Rect, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+HeightProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::RectPropertyBag_HeightProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+HeightProperty")]
impl crate::Unity::Properties::Internal::RectPropertyBag_HeightProperty {
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
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+HeightProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectPropertyBag_HeightProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+WidthProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct RectPropertyBag_WidthProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<crate::UnityEngine::Rect, f32>,
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+WidthProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::RectPropertyBag_WidthProperty =>
    "Unity.Properties.Internal"."RectPropertyBag/WidthProperty"
);
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+WidthProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::RectPropertyBag_WidthProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Rect, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+WidthProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::RectPropertyBag_WidthProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+WidthProperty")]
impl crate::Unity::Properties::Internal::RectPropertyBag_WidthProperty {
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
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+WidthProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectPropertyBag_WidthProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+XProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct RectPropertyBag_XProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<crate::UnityEngine::Rect, f32>,
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+XProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::RectPropertyBag_XProperty =>
    "Unity.Properties.Internal"."RectPropertyBag/XProperty"
);
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+XProperty")]
impl std::ops::Deref for crate::Unity::Properties::Internal::RectPropertyBag_XProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Rect, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+XProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::RectPropertyBag_XProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+XProperty")]
impl crate::Unity::Properties::Internal::RectPropertyBag_XProperty {
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
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+XProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectPropertyBag_XProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+YProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct RectPropertyBag_YProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<crate::UnityEngine::Rect, f32>,
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+YProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::RectPropertyBag_YProperty =>
    "Unity.Properties.Internal"."RectPropertyBag/YProperty"
);
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+YProperty")]
impl std::ops::Deref for crate::Unity::Properties::Internal::RectPropertyBag_YProperty {
    type Target = crate::Unity::Properties::Property_2<crate::UnityEngine::Rect, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+YProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::RectPropertyBag_YProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+YProperty")]
impl crate::Unity::Properties::Internal::RectPropertyBag_YProperty {
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
#[cfg(feature = "Unity+Properties+Internal+RectPropertyBag+YProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::RectPropertyBag_YProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
