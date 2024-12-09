#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::CollectionUtils =>
    "Newtonsoft.Json.Utilities"."CollectionUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl crate::Newtonsoft::Json::Utilities::CollectionUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
    pub type EmptyArrayContainer_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<
        T,
    >;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionUtils_EmptyArrayContainer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1 < T > =>
    "Newtonsoft.Json.Utilities"."CollectionUtils/EmptyArrayContainer`1" < T >
);
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
