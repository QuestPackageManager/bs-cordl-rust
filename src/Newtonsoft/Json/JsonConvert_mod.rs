#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonConvert {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::JsonConvert {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json";
    const CLASS_NAME: &'static str = "JsonConvert";
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
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonConvert {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonConvert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
impl crate::Newtonsoft::Json::JsonConvert {
    pub fn DeserializeAnonymousType_Il2CppString_T0<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        anonymousTypeObject: T,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeAnonymousType", (value, anonymousTypeObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeAnonymousType_JsonSerializerSettings1<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        anonymousTypeObject: T,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeAnonymousType", (value, anonymousTypeObject, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Il2CppArray4<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        converters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (value, converters))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Il2CppString0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Il2CppString3<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_JsonSerializerSettings1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (value, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_JsonSerializerSettings5<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (value, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Type2(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (value, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Type_Il2CppArray6(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        converters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (value, _cordl_type, converters))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Type_JsonSerializerSettings7(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (value, _cordl_type, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXNode_Il2CppString0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeXNode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXNode_Il2CppString1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deserializeRootElementName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeXNode", (value, deserializeRootElementName))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXNode_Il2CppString__cordl_bool2(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deserializeRootElementName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        writeArrayAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DeserializeXNode",
                (value, deserializeRootElementName, writeArrayAttribute),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXNode_Il2CppString__cordl_bool__cordl_bool3(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deserializeRootElementName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        writeArrayAttribute: bool,
        encodeSpecialCharacters: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DeserializeXNode",
                (
                    value,
                    deserializeRootElementName,
                    writeArrayAttribute,
                    encodeSpecialCharacters,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXmlNode_Il2CppString0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeXmlNode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXmlNode_Il2CppString1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deserializeRootElementName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeXmlNode", (value, deserializeRootElementName))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXmlNode_Il2CppString__cordl_bool2(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deserializeRootElementName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        writeArrayAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DeserializeXmlNode",
                (value, deserializeRootElementName, writeArrayAttribute),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXmlNode_Il2CppString__cordl_bool__cordl_bool3(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deserializeRootElementName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        writeArrayAttribute: bool,
        encodeSpecialCharacters: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DeserializeXmlNode",
                (
                    value,
                    deserializeRootElementName,
                    writeArrayAttribute,
                    encodeSpecialCharacters,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDecimalPlace_Il2CppString1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureDecimalPlace", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDecimalPlace_f64_Il2CppString0(
        value: f64,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureDecimalPlace", (value, text))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureFloatFormat(
        value: f64,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        floatFormatHandling: crate::Newtonsoft::Json::FloatFormatHandling,
        quoteChar: char,
        nullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EnsureFloatFormat",
                (value, text, floatFormatHandling, quoteChar, nullable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateObject_Il2CppString_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopulateObject", (value, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateObject_JsonSerializerSettings1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopulateObject", (value, target, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObjectInternal(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        jsonSerializer: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObjectInternal", (value, _cordl_type, jsonSerializer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Formatting1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        formatting: crate::Newtonsoft::Json::Formatting,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (value, formatting))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Formatting_Il2CppArray3(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        formatting: crate::Newtonsoft::Json::Formatting,
        converters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (value, formatting, converters))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Formatting_JsonSerializerSettings6(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        formatting: crate::Newtonsoft::Json::Formatting,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (value, formatting, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Il2CppArray2(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        converters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (value, converters))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_JsonSerializerSettings4(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (value, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Type_Formatting_JsonSerializerSettings7(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        formatting: crate::Newtonsoft::Json::Formatting,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (value, _cordl_type, formatting, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Type_JsonSerializerSettings5(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (value, _cordl_type, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXNode_Formatting1(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>,
        formatting: crate::Newtonsoft::Json::Formatting,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeXNode", (node, formatting))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXNode_Formatting__cordl_bool2(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>,
        formatting: crate::Newtonsoft::Json::Formatting,
        omitRootObject: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeXNode", (node, formatting, omitRootObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXNode_XObject0(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeXNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXmlNode_Formatting1(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        formatting: crate::Newtonsoft::Json::Formatting,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeXmlNode", (node, formatting))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXmlNode_Formatting__cordl_bool2(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        formatting: crate::Newtonsoft::Json::Formatting,
        omitRootObject: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeXmlNode", (node, formatting, omitRootObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXmlNode_XmlNode0(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeXmlNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToStringInternal(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToStringInternal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_DateTime0(
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_DateTimeOffset2(
        value: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_DateTimeOffset_DateFormatHandling3(
        value: crate::System::DateTimeOffset,
        format: crate::Newtonsoft::Json::DateFormatHandling,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_DateTime_DateFormatHandling_DateTimeZoneHandling1(
        value: crate::System::DateTime,
        format: crate::Newtonsoft::Json::DateFormatHandling,
        timeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, format, timeZoneHandling))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Decimal19(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Enum6(
        value: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Guid20(
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Guid__cordl_char21(
        value: crate::System::Guid,
        quoteChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, quoteChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppObject29(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString26(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString__cordl_char27(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, delimiter))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString__cordl_char_StringEscapeHandling28(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, delimiter, stringEscapeHandling))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_TimeSpan22(
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_TimeSpan__cordl_char23(
        value: crate::System::TimeSpan,
        quoteChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, quoteChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Uri24(
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Uri__cordl_char25(
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        quoteChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, quoteChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_bool4(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_char5(
        value: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_f32_13(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_f32_FloatFormatHandling__cordl_char__cordl_bool14(
        value: f32,
        floatFormatHandling: crate::Newtonsoft::Json::FloatFormatHandling,
        quoteChar: char,
        nullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, floatFormatHandling, quoteChar, nullable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_f64_15(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_f64_FloatFormatHandling__cordl_char__cordl_bool16(
        value: f64,
        floatFormatHandling: crate::Newtonsoft::Json::FloatFormatHandling,
        quoteChar: char,
        nullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, floatFormatHandling, quoteChar, nullable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i16_8(
        value: i16,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_7(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i64_11(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i8_18(
        value: i8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u16_9(
        value: u16,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u32_10(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u64_12(
        value: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u8_17(
        value: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::JsonSerializerSettings,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::JsonSerializerSettings,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultSettings(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::JsonSerializerSettings,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultSettings", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::JsonConvert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
