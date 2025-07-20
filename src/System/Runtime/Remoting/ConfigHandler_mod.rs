#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub typeEntries: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub channelInstances: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ArrayList,
    >,
    pub currentChannel: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::ChannelData,
    >,
    pub currentProviderData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Stack,
    >,
    pub currentClientUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub appName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub currentXmlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub onlyDelayedChannels: bool,
}
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::ConfigHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting";
    const CLASS_NAME: &'static str = "ConfigHandler";
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
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
impl std::ops::Deref for crate::System::Runtime::Remoting::ConfigHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::ConfigHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
impl crate::System::Runtime::Remoting::ConfigHandler {
    pub fn CheckPath(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("CheckPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "CheckPath", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (path))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractAssembly(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ExtractAssembly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ExtractAssembly", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNotNull(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Mono::Xml::SmallXmlParser_IAttrList,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetNotNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "GetNotNull", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (attrs, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        onlyDelayedChannels: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (onlyDelayedChannels))?;
        Ok(__cordl_object.into())
    }
    pub fn OnChars(
        &mut self,
        ch: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "OnChars", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ch))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEndElement(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnEndElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "OnEndElement", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEndParsing(
        &mut self,
        parser: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnEndParsing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "OnEndParsing", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parser))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnIgnorableWhitespace(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnIgnorableWhitespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "OnIgnorableWhitespace",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (s))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnProcessingInstruction(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnProcessingInstruction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "OnProcessingInstruction",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, text))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnStartElement(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnStartElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "OnStartElement", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnStartParsing(
        &mut self,
        parser: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnStartParsing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "OnStartParsing", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parser))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseElement(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ParseElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ParseElement", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseTime(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::TimeSpan,
                1usize,
            >("ParseTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ParseTime", 1usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked(self, (s))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadChannel(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
        isTemplate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Mono::Xml::SmallXmlParser_IAttrList,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadChannel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadChannel", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attrs, isTemplate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadClientActivated(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadClientActivated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadClientActivated",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadClientWellKnown(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadClientWellKnown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadClientWellKnown",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCustomProviderData(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadCustomProviderData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadCustomProviderData",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadInteropXml(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
        isElement: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Mono::Xml::SmallXmlParser_IAttrList,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadInteropXml")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadInteropXml", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attrs, isElement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadLifetine(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadLifetine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadLifetine", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPreload(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadPreload")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadPreload", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadProvider(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
        isTemplate: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ProviderData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::Mono::Xml::SmallXmlParser_IAttrList,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ProviderData,
                >,
                3usize,
            >("ReadProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadProvider", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ProviderData,
        > = unsafe { method.invoke_unchecked(self, (name, attrs, isTemplate))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadServiceActivated(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadServiceActivated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadServiceActivated",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadServiceWellKnown(
        &mut self,
        attrs: quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Xml::SmallXmlParser_IAttrList>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadServiceWellKnown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ReadServiceWellKnown",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (attrs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidatePath(
        &mut self,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        paths: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ValidatePath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), "ValidatePath", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element, paths))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        onlyDelayedChannels: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::ConfigHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::ConfigHandler as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (onlyDelayedChannels))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::ConfigHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
impl AsRef<crate::Mono::Xml::SmallXmlParser_IContentHandler>
for crate::System::Runtime::Remoting::ConfigHandler {
    fn as_ref(&self) -> &crate::Mono::Xml::SmallXmlParser_IContentHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
impl AsMut<crate::Mono::Xml::SmallXmlParser_IContentHandler>
for crate::System::Runtime::Remoting::ConfigHandler {
    fn as_mut(&mut self) -> &mut crate::Mono::Xml::SmallXmlParser_IContentHandler {
        unsafe { std::mem::transmute(self) }
    }
}
