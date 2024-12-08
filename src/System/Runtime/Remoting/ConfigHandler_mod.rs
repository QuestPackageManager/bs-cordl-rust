#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigHandler {
    __cordl_parent: crate::System::Object,
    pub typeEntries: *mut crate::System::Collections::ArrayList,
    pub channelInstances: *mut crate::System::Collections::ArrayList,
    pub currentChannel: *mut crate::System::Runtime::Remoting::ChannelData,
    pub currentProviderData: *mut crate::System::Collections::Stack,
    pub currentClientUrl: *mut crate::System::String,
    pub appName: *mut crate::System::String,
    pub currentXmlPath: *mut crate::System::String,
    pub onlyDelayedChannels: bool,
}
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::ConfigHandler =>
    "System.Runtime.Remoting"."ConfigHandler"
);
#[cfg(feature = "System+Runtime+Remoting+ConfigHandler")]
impl std::ops::Deref for crate::System::Runtime::Remoting::ConfigHandler {
    type Target = crate::System::Object;
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
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckPath", (path))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractAssembly(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ExtractAssembly", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetNotNull(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetNotNull", (attrs, name))?;
        Ok(__cordl_ret)
    }
    pub fn New(onlyDelayedChannels: bool) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (onlyDelayedChannels))?;
        Ok(__cordl_object)
    }
    pub fn OnChars(
        &mut self,
        ch: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnChars", (ch))?;
        Ok(__cordl_ret)
    }
    pub fn OnEndElement(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEndElement", (name))?;
        Ok(__cordl_ret)
    }
    pub fn OnEndParsing(
        &mut self,
        parser: *mut crate::Mono::Xml::SmallXmlParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEndParsing", (parser))?;
        Ok(__cordl_ret)
    }
    pub fn OnIgnorableWhitespace(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnIgnorableWhitespace", (s))?;
        Ok(__cordl_ret)
    }
    pub fn OnProcessingInstruction(
        &mut self,
        name: *mut crate::System::String,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnProcessingInstruction", (name, text))?;
        Ok(__cordl_ret)
    }
    pub fn OnStartElement(
        &mut self,
        name: *mut crate::System::String,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStartElement", (name, attrs))?;
        Ok(__cordl_ret)
    }
    pub fn OnStartParsing(
        &mut self,
        parser: *mut crate::Mono::Xml::SmallXmlParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStartParsing", (parser))?;
        Ok(__cordl_ret)
    }
    pub fn ParseElement(
        &mut self,
        name: *mut crate::System::String,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseElement", (name, attrs))?;
        Ok(__cordl_ret)
    }
    pub fn ParseTime(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("ParseTime", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ReadChannel(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
        isTemplate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadChannel", (attrs, isTemplate))?;
        Ok(__cordl_ret)
    }
    pub fn ReadClientActivated(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadClientActivated", (attrs))?;
        Ok(__cordl_ret)
    }
    pub fn ReadClientWellKnown(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadClientWellKnown", (attrs))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCustomProviderData(
        &mut self,
        name: *mut crate::System::String,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadCustomProviderData", (name, attrs))?;
        Ok(__cordl_ret)
    }
    pub fn ReadInteropXml(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
        isElement: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadInteropXml", (attrs, isElement))?;
        Ok(__cordl_ret)
    }
    pub fn ReadLifetine(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadLifetine", (attrs))?;
        Ok(__cordl_ret)
    }
    pub fn ReadPreload(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadPreload", (attrs))?;
        Ok(__cordl_ret)
    }
    pub fn ReadProvider(
        &mut self,
        name: *mut crate::System::String,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
        isTemplate: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::ProviderData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::ProviderData = __cordl_object
            .invoke("ReadProvider", (name, attrs, isTemplate))?;
        Ok(__cordl_ret)
    }
    pub fn ReadServiceActivated(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadServiceActivated", (attrs))?;
        Ok(__cordl_ret)
    }
    pub fn ReadServiceWellKnown(
        &mut self,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadServiceWellKnown", (attrs))?;
        Ok(__cordl_ret)
    }
    pub fn ValidatePath(
        &mut self,
        element: *mut crate::System::String,
        paths: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidatePath", (element, paths))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        onlyDelayedChannels: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (onlyDelayedChannels))?;
        Ok(__cordl_ret)
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
