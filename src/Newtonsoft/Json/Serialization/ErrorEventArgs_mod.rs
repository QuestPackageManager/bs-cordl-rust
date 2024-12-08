#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct ErrorEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _CurrentObject_k__BackingField: *mut crate::System::Object,
    pub _ErrorContext_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::ErrorContext,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Serialization::ErrorEventArgs
    => "Newtonsoft.Json.Serialization"."ErrorEventArgs"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorEventArgs")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::ErrorEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorEventArgs")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::ErrorEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorEventArgs")]
impl crate::Newtonsoft::Json::Serialization::ErrorEventArgs {
    pub fn get_ErrorContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ErrorContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ErrorContext = __cordl_object
            .invoke("get_ErrorContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        currentObject: *mut crate::System::Object,
        errorContext: *mut crate::Newtonsoft::Json::Serialization::ErrorContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (currentObject, errorContext))?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_CurrentObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        currentObject: *mut crate::System::Object,
        errorContext: *mut crate::Newtonsoft::Json::Serialization::ErrorContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (currentObject, errorContext))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::ErrorEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
