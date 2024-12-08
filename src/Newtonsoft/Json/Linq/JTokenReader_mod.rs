#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
#[repr(C)]
#[derive(Debug)]
pub struct JTokenReader {
    __cordl_parent: crate::Newtonsoft::Json::JsonReader,
    pub _root: *mut crate::Newtonsoft::Json::Linq::JToken,
    pub _initialPath: *mut crate::System::String,
    pub _parent: *mut crate::Newtonsoft::Json::Linq::JToken,
    pub _current: *mut crate::Newtonsoft::Json::Linq::JToken,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JTokenReader =>
    "Newtonsoft.Json.Linq"."JTokenReader"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JTokenReader {
    type Target = crate::Newtonsoft::Json::JsonReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JTokenReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
impl crate::Newtonsoft::Json::Linq::JTokenReader {
    pub fn Newtonsoft_Json_IJsonLineInfo_HasLineInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.HasLineInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetToken(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToken", (token))?;
        Ok(__cordl_ret)
    }
    pub fn ReadOver(
        &mut self,
        t: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadOver", (t))?;
        Ok(__cordl_ret)
    }
    pub fn ReadInto(
        &mut self,
        c: *mut crate::Newtonsoft::Json::Linq::JContainer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadInto", (c))?;
        Ok(__cordl_ret)
    }
    pub fn SafeToString(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("SafeToString", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetEndToken(
        &mut self,
        c: *mut crate::Newtonsoft::Json::Linq::JContainer,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::JsonToken>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::Newtonsoft::Json::JsonToken> = __cordl_object
            .invoke("GetEndToken", (c))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JToken0(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
        initialPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token, initialPath))?;
        Ok(__cordl_ret)
    }
    pub fn Newtonsoft_Json_IJsonLineInfo_get_LineNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.get_LineNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn Newtonsoft_Json_IJsonLineInfo_get_LinePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.get_LinePosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetEnd(
        &mut self,
        c: *mut crate::Newtonsoft::Json::Linq::JContainer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetEnd", (c))?;
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
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadToEnd(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadToEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("get_CurrentToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_JToken0(
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
        initialPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token, initialPath))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Linq::JTokenReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
