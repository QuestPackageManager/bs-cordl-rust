#[cfg(feature = "Newtonsoft+Json+JsonConverterCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonConverterCollection {
    __cordl_parent: crate::System::Collections::ObjectModel::Collection_1<
        *mut crate::Newtonsoft::Json::JsonConverter,
    >,
}
#[cfg(feature = "Newtonsoft+Json+JsonConverterCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonConverterCollection =>
    "Newtonsoft.Json"."JsonConverterCollection"
);
#[cfg(feature = "Newtonsoft+Json+JsonConverterCollection")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonConverterCollection {
    type Target = crate::System::Collections::ObjectModel::Collection_1<
        *mut crate::Newtonsoft::Json::JsonConverter,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConverterCollection")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonConverterCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConverterCollection")]
impl crate::Newtonsoft::Json::JsonConverterCollection {
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
#[cfg(feature = "Newtonsoft+Json+JsonConverterCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::JsonConverterCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
