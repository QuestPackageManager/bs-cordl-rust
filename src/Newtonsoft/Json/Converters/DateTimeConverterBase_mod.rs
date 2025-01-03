#[cfg(feature = "Newtonsoft+Json+Converters+DateTimeConverterBase")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeConverterBase {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter,
}
#[cfg(feature = "Newtonsoft+Json+Converters+DateTimeConverterBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::DateTimeConverterBase => "Newtonsoft.Json.Converters"
    ."DateTimeConverterBase"
);
#[cfg(feature = "Newtonsoft+Json+Converters+DateTimeConverterBase")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::DateTimeConverterBase {
    type Target = crate::Newtonsoft::Json::JsonConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DateTimeConverterBase")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::DateTimeConverterBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DateTimeConverterBase")]
impl crate::Newtonsoft::Json::Converters::DateTimeConverterBase {
    pub fn CanConvert(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanConvert", (objectType))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "Newtonsoft+Json+Converters+DateTimeConverterBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::DateTimeConverterBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
