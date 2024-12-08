#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCasePropertyNamesContractResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct CamelCasePropertyNamesContractResolver {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::DefaultContractResolver,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCasePropertyNamesContractResolver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::CamelCasePropertyNamesContractResolver =>
    "Newtonsoft.Json.Serialization"."CamelCasePropertyNamesContractResolver"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCasePropertyNamesContractResolver")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::CamelCasePropertyNamesContractResolver {
    type Target = crate::Newtonsoft::Json::Serialization::DefaultContractResolver;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCasePropertyNamesContractResolver")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::CamelCasePropertyNamesContractResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCasePropertyNamesContractResolver")]
impl crate::Newtonsoft::Json::Serialization::CamelCasePropertyNamesContractResolver {
    pub fn GetNameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::DefaultJsonNameTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::DefaultJsonNameTable = __cordl_object
            .invoke("GetNameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ResolveContract(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonContract = __cordl_object
            .invoke("ResolveContract", (_cordl_type))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCasePropertyNamesContractResolver")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::CamelCasePropertyNamesContractResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
