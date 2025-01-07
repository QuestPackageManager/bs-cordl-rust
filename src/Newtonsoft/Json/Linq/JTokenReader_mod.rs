#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
#[repr(C)]
#[derive(Debug)]
pub struct JTokenReader {
    __cordl_parent: crate::Newtonsoft::Json::JsonReader,
    pub _root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    pub _initialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _parent: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    pub _current: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::Linq::JTokenReader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Linq";
    const CLASS_NAME: &'static str = "JTokenReader";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn GetEndToken(
        &mut self,
        c: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JContainer>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::JsonToken>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::Newtonsoft::Json::JsonToken> = __cordl_object
            .invoke("GetEndToken", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString1(
        token: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        initialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token, initialPath))?;
        Ok(__cordl_object.into())
    }
    pub fn New_JToken0(
        token: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token))?;
        Ok(__cordl_object.into())
    }
    pub fn Newtonsoft_Json_IJsonLineInfo_HasLineInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.HasLineInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Newtonsoft_Json_IJsonLineInfo_get_LineNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.get_LineNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Newtonsoft_Json_IJsonLineInfo_get_LinePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.get_LinePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadInto(
        &mut self,
        c: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JContainer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadInto", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadOver(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadOver", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadToEnd(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadToEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SafeToString(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("SafeToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEnd(
        &mut self,
        c: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JContainer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetEnd", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetToken(
        &mut self,
        token: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToken", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        token: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        initialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token, initialPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_JToken0(
        &mut self,
        token: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JToken,
        > = __cordl_object.invoke("get_CurrentToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Path", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
impl AsRef<crate::Newtonsoft::Json::IJsonLineInfo>
for crate::Newtonsoft::Json::Linq::JTokenReader {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::IJsonLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenReader")]
impl AsMut<crate::Newtonsoft::Json::IJsonLineInfo>
for crate::Newtonsoft::Json::Linq::JTokenReader {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::IJsonLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
