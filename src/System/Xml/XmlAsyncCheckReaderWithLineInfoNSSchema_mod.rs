#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAsyncCheckReaderWithLineInfoNSSchema {
    __cordl_parent: crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNS,
    pub readerAsIXmlSchemaInfo: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::IXmlSchemaInfo,
    >,
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlAsyncCheckReaderWithLineInfoNSSchema";
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
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl std::ops::Deref for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    type Target = crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNS;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl std::ops::DerefMut for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    pub fn New(
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_IsDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("System.Xml.Schema.IXmlSchemaInfo.get_IsDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Schema.IXmlSchemaInfo.get_IsDefault", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_IsNil(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("System.Xml.Schema.IXmlSchemaInfo.get_IsNil")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Schema.IXmlSchemaInfo.get_IsNil", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_MemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaSimpleType,
                        >,
                        0usize,
                    >("System.Xml.Schema.IXmlSchemaInfo.get_MemberType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Schema.IXmlSchemaInfo.get_MemberType", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_SchemaAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaAttribute,
                        >,
                        0usize,
                    >("System.Xml.Schema.IXmlSchemaInfo.get_SchemaAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Schema.IXmlSchemaInfo.get_SchemaAttribute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttribute,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_SchemaElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaElement,
                        >,
                        0usize,
                    >("System.Xml.Schema.IXmlSchemaInfo.get_SchemaElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Schema.IXmlSchemaInfo.get_SchemaElement", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_SchemaType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaType,
                        >,
                        0usize,
                    >("System.Xml.Schema.IXmlSchemaInfo.get_SchemaType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Schema.IXmlSchemaInfo.get_SchemaType", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaType,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_Validity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaValidity> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Xml::Schema::XmlSchemaValidity,
                        0usize,
                    >("System.Xml.Schema.IXmlSchemaInfo.get_Validity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Schema.IXmlSchemaInfo.get_Validity", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaValidity = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl AsRef<crate::System::Xml::Schema::IXmlSchemaInfo>
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    fn as_ref(&self) -> &crate::System::Xml::Schema::IXmlSchemaInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl AsMut<crate::System::Xml::Schema::IXmlSchemaInfo>
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    fn as_mut(&mut self) -> &mut crate::System::Xml::Schema::IXmlSchemaInfo {
        unsafe { std::mem::transmute(self) }
    }
}
