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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, T),
                T,
                2usize,
            >("DeserializeAnonymousType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeAnonymousType", 2usize
                )
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (value, anonymousTypeObject))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    T,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                T,
                3usize,
            >("DeserializeAnonymousType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeAnonymousType", 3usize
                )
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (value, anonymousTypeObject, settings))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonConverter,
                            >,
                        >,
                    >,
                ),
                T,
                2usize,
            >("DeserializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeObject", 2usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (value, converters)) };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Il2CppString0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("DeserializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeObject", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Il2CppString3<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                T,
                1usize,
            >("DeserializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeObject", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (value)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("DeserializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, settings)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                T,
                2usize,
            >("DeserializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeObject", 2usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (value, settings)) };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject_Type2(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("DeserializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, _cordl_type)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonConverter,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("DeserializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeObject", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, _cordl_type, converters)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("DeserializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeObject", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, _cordl_type, settings)) };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXNode_Il2CppString0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
                1usize,
            >("DeserializeXNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeXNode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = unsafe { method.invoke_unchecked((), (value)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
                2usize,
            >("DeserializeXNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeXNode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = unsafe { method.invoke_unchecked((), (value, deserializeRootElementName)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
                3usize,
            >("DeserializeXNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeXNode", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, deserializeRootElementName, writeArrayAttribute),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
                4usize,
            >("DeserializeXNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeXNode", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        value,
                        deserializeRootElementName,
                        writeArrayAttribute,
                        encodeSpecialCharacters,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeXmlNode_Il2CppString0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
                1usize,
            >("DeserializeXmlNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeXmlNode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = unsafe {
            method.invoke_unchecked((), (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
                2usize,
            >("DeserializeXmlNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeXmlNode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = unsafe {
            method.invoke_unchecked((), (value, deserializeRootElementName))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
                3usize,
            >("DeserializeXmlNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeXmlNode", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, deserializeRootElementName, writeArrayAttribute),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
                4usize,
            >("DeserializeXmlNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeserializeXmlNode", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument> = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        value,
                        deserializeRootElementName,
                        writeArrayAttribute,
                        encodeSpecialCharacters,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDecimalPlace_Il2CppString1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("EnsureDecimalPlace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureDecimalPlace", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (text)) };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDecimalPlace_f64_Il2CppString0(
        value: f64,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("EnsureDecimalPlace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureDecimalPlace", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, text)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Newtonsoft::Json::FloatFormatHandling,
                    char,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                5usize,
            >("EnsureFloatFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureFloatFormat", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, text, floatFormatHandling, quoteChar, nullable),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateObject_Il2CppString_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PopulateObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PopulateObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, target))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateObject_JsonSerializerSettings1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PopulateObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PopulateObject", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, target, settings))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("SerializeObjectInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObjectInternal", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, _cordl_type, jsonSerializer)) };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Formatting1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        formatting: crate::Newtonsoft::Json::Formatting,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Newtonsoft::Json::Formatting,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("SerializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, formatting)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Newtonsoft::Json::Formatting,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonConverter,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("SerializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObject", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, formatting, converters)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Newtonsoft::Json::Formatting,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("SerializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObject", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, formatting, settings)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonConverter,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("SerializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, converters)) };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("SerializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObject", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("SerializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, settings)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    crate::Newtonsoft::Json::Formatting,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                4usize,
            >("SerializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObject", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method.invoke_unchecked((), (value, _cordl_type, formatting, settings))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("SerializeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeObject", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, _cordl_type, settings)) };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXNode_Formatting1(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>,
        formatting: crate::Newtonsoft::Json::Formatting,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>,
                    crate::Newtonsoft::Json::Formatting,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("SerializeXNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeXNode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (node, formatting)) };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXNode_Formatting__cordl_bool2(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>,
        formatting: crate::Newtonsoft::Json::Formatting,
        omitRootObject: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>,
                    crate::Newtonsoft::Json::Formatting,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("SerializeXNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeXNode", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (node, formatting, omitRootObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXNode_XObject0(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObject>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("SerializeXNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeXNode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXmlNode_Formatting1(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        formatting: crate::Newtonsoft::Json::Formatting,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                    crate::Newtonsoft::Json::Formatting,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("SerializeXmlNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeXmlNode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (node, formatting)) };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXmlNode_Formatting__cordl_bool2(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        formatting: crate::Newtonsoft::Json::Formatting,
        omitRootObject: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                    crate::Newtonsoft::Json::Formatting,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("SerializeXmlNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeXmlNode", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (node, formatting, omitRootObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeXmlNode_XmlNode0(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("SerializeXmlNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SerializeXmlNode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToStringInternal(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Numerics::BigInteger),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToStringInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToStringInternal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_DateTime0(
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_DateTimeOffset2(
        value: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTimeOffset),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_DateTimeOffset_DateFormatHandling3(
        value: crate::System::DateTimeOffset,
        format: crate::Newtonsoft::Json::DateFormatHandling,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::DateTimeOffset,
                    crate::Newtonsoft::Json::DateFormatHandling,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_DateTime_DateFormatHandling_DateTimeZoneHandling1(
        value: crate::System::DateTime,
        format: crate::Newtonsoft::Json::DateFormatHandling,
        timeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::DateTime,
                    crate::Newtonsoft::Json::DateFormatHandling,
                    crate::Newtonsoft::Json::DateTimeZoneHandling,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format, timeZoneHandling)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Decimal19(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Decimal),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Enum6(
        value: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Enum>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Guid20(
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Guid),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Guid__cordl_char21(
        value: crate::System::Guid,
        quoteChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Guid, char),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, quoteChar)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppObject29(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString26(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString__cordl_char27(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, char),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, delimiter)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString__cordl_char_StringEscapeHandling28(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    char,
                    crate::Newtonsoft::Json::StringEscapeHandling,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method.invoke_unchecked((), (value, delimiter, stringEscapeHandling))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_TimeSpan22(
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::TimeSpan),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_TimeSpan__cordl_char23(
        value: crate::System::TimeSpan,
        quoteChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::TimeSpan, char),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, quoteChar)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Uri24(
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Uri__cordl_char25(
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        quoteChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>, char),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, quoteChar)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_bool4(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_char5(
        value: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (char),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_f32_13(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, crate::Newtonsoft::Json::FloatFormatHandling, char, bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                4usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method
                .invoke_unchecked((), (value, floatFormatHandling, quoteChar, nullable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_f64_15(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64, crate::Newtonsoft::Json::FloatFormatHandling, char, bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                4usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method
                .invoke_unchecked((), (value, floatFormatHandling, quoteChar, nullable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i16_8(
        value: i16,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i16),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_7(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i64_11(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i8_18(
        value: i8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i8),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u16_9(
        value: u16,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u16),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u32_10(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u64_12(
        value: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u8_17(
        value: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u8),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Func_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::JsonSerializerSettings,
                        >,
                    >,
                >,
                0usize,
            >("get_DefaultSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_DefaultSettings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::JsonSerializerSettings,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Func_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::JsonSerializerSettings,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_DefaultSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_DefaultSettings", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))
        };
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
