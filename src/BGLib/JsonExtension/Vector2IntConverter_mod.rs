#[cfg(feature = "BGLib+JsonExtension+Vector2IntConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector2IntConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter_1<
        crate::UnityEngine::Vector2Int,
    >,
}
#[cfg(feature = "BGLib+JsonExtension+Vector2IntConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::JsonExtension::Vector2IntConverter =>
    "BGLib.JsonExtension"."Vector2IntConverter"
);
#[cfg(feature = "BGLib+JsonExtension+Vector2IntConverter")]
impl std::ops::Deref for crate::BGLib::JsonExtension::Vector2IntConverter {
    type Target = crate::Newtonsoft::Json::JsonConverter_1<
        crate::UnityEngine::Vector2Int,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+Vector2IntConverter")]
impl std::ops::DerefMut for crate::BGLib::JsonExtension::Vector2IntConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+Vector2IntConverter")]
impl crate::BGLib::JsonExtension::Vector2IntConverter {
    pub fn WriteJson(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: crate::UnityEngine::Vector2Int,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
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
    pub fn ReadJson(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        existingValue: crate::UnityEngine::Vector2Int,
        hasExistingValue: bool,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2Int = __cordl_object
            .invoke(
                "ReadJson",
                (reader, objectType, existingValue, hasExistingValue, serializer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BGLib+JsonExtension+Vector2IntConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::JsonExtension::Vector2IntConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
