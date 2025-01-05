#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter_1<crate::UnityEngine::Color>,
}
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::JsonExtension::ColorConverter =>
    "BGLib.JsonExtension"."ColorConverter"
);
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
impl std::ops::Deref for crate::BGLib::JsonExtension::ColorConverter {
    type Target = crate::Newtonsoft::Json::JsonConverter_1<crate::UnityEngine::Color>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
impl std::ops::DerefMut for crate::BGLib::JsonExtension::ColorConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
impl crate::BGLib::JsonExtension::ColorConverter {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadJson(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        existingValue: crate::UnityEngine::Color,
        hasExistingValue: bool,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke(
                "ReadJson",
                (reader, objectType, existingValue, hasExistingValue, serializer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteJson(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: crate::UnityEngine::Color,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::JsonExtension::ColorConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
