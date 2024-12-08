#[cfg(feature = "Newtonsoft+Json+Schema+ValidationEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _ex: *mut crate::Newtonsoft::Json::Schema::JsonSchemaException,
}
#[cfg(feature = "Newtonsoft+Json+Schema+ValidationEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::ValidationEventArgs =>
    "Newtonsoft.Json.Schema"."ValidationEventArgs"
);
#[cfg(feature = "Newtonsoft+Json+Schema+ValidationEventArgs")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::ValidationEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+ValidationEventArgs")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::ValidationEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+ValidationEventArgs")]
impl crate::Newtonsoft::Json::Schema::ValidationEventArgs {
    pub fn get_Exception(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaException,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaException = __cordl_object
            .invoke("get_Exception", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        ex: *mut crate::Newtonsoft::Json::Schema::JsonSchemaException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ex))?;
        Ok(__cordl_ret)
    }
    pub fn get_Path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Path", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Message", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        ex: *mut crate::Newtonsoft::Json::Schema::JsonSchemaException,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ex))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+ValidationEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::ValidationEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
