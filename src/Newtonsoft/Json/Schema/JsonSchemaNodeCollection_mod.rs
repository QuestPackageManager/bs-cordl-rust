#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNodeCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaNodeCollection {
    __cordl_parent: crate::System::Collections::ObjectModel::KeyedCollection_2<
        *mut crate::System::String,
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNodeCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Schema::JsonSchemaNodeCollection => "Newtonsoft.Json.Schema"
    ."JsonSchemaNodeCollection"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNodeCollection")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaNodeCollection {
    type Target = crate::System::Collections::ObjectModel::KeyedCollection_2<
        *mut crate::System::String,
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNodeCollection")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaNodeCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNodeCollection")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaNodeCollection {
    pub fn GetKeyForItem(
        &mut self,
        item: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetKeyForItem", (item))?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNodeCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaNodeCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
