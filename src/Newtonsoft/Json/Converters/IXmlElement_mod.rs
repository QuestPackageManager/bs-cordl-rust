#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
#[repr(C)]
#[derive(Debug)]
pub struct IXmlElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Converters::IXmlElement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Converters";
    const CLASS_NAME: &'static str = "IXmlElement";
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
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::IXmlElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::IXmlElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl crate::Newtonsoft::Json::Converters::IXmlElement {
    pub fn GetPrefixOfNamespace(
        &mut self,
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::IXmlElement as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetPrefixOfNamespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::IXmlElement as
                    quest_hook::libil2cpp::Type > ::class(), "GetPrefixOfNamespace",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (namespaceUri))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributeNode(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::IXmlElement as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Converters::IXmlNode,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetAttributeNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::IXmlElement as
                    quest_hook::libil2cpp::Type > ::class(), "SetAttributeNode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attribute))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::IXmlElement as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsEmpty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::IXmlElement as
                    quest_hook::libil2cpp::Type > ::class(), "get_IsEmpty", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::IXmlElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl AsRef<crate::Newtonsoft::Json::Converters::IXmlNode>
for crate::Newtonsoft::Json::Converters::IXmlElement {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Converters::IXmlNode {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl AsMut<crate::Newtonsoft::Json::Converters::IXmlNode>
for crate::Newtonsoft::Json::Converters::IXmlElement {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::Converters::IXmlNode {
        unsafe { std::mem::transmute(self) }
    }
}
