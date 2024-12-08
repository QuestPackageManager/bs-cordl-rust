#[cfg(feature = "Newtonsoft+Json+Serialization+JsonStringContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonStringContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonStringContract")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonStringContract =>
    "Newtonsoft.Json.Serialization"."JsonStringContract"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonStringContract")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonStringContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonStringContract")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::JsonStringContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonStringContract")]
impl crate::Newtonsoft::Json::Serialization::JsonStringContract {
    pub fn _ctor(
        &mut self,
        underlyingType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (underlyingType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        underlyingType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonStringContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonStringContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
