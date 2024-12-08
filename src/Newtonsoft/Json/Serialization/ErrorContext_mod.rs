#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorContext")]
#[repr(C)]
#[derive(Debug)]
pub struct ErrorContext {
    __cordl_parent: crate::System::Object,
    pub _Traced_k__BackingField: bool,
    pub _Error_k__BackingField: *mut crate::System::Exception,
    pub _OriginalObject_k__BackingField: *mut crate::System::Object,
    pub _Member_k__BackingField: *mut crate::System::Object,
    pub _Path_k__BackingField: *mut crate::System::String,
    pub _Handled_k__BackingField: bool,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Serialization::ErrorContext =>
    "Newtonsoft.Json.Serialization"."ErrorContext"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorContext")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::ErrorContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorContext")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::ErrorContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorContext")]
impl crate::Newtonsoft::Json::Serialization::ErrorContext {
    pub fn get_Traced(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Traced", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        originalObject: *mut crate::System::Object,
        member: *mut crate::System::Object,
        path: *mut crate::System::String,
        error: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (originalObject, member, path, error))?;
        Ok(__cordl_ret)
    }
    pub fn set_Handled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Handled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_OriginalObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_OriginalObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Error(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("get_Error", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Traced(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Traced", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Handled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Handled", ())?;
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
    pub fn get_Member(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Member", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        originalObject: *mut crate::System::Object,
        member: *mut crate::System::Object,
        path: *mut crate::System::String,
        error: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (originalObject, member, path, error))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ErrorContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::ErrorContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
