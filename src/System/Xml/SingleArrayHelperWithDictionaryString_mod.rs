#[cfg(feature = "cordl_class_System+Xml+SingleArrayHelperWithDictionaryString")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SingleArrayHelperWithDictionaryString {
    __cordl_parent: crate::System::Xml::ArrayHelper_2<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        f32,
    >,
}
#[cfg(feature = "cordl_class_System+Xml+SingleArrayHelperWithDictionaryString")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Xml::SingleArrayHelperWithDictionaryString
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "SingleArrayHelperWithDictionaryString";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Xml+SingleArrayHelperWithDictionaryString")]
impl std::ops::Deref for crate::System::Xml::SingleArrayHelperWithDictionaryString {
    type Target = crate::System::Xml::ArrayHelper_2<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        f32,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SingleArrayHelperWithDictionaryString")]
impl std::ops::DerefMut for crate::System::Xml::SingleArrayHelperWithDictionaryString {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SingleArrayHelperWithDictionaryString")]
impl crate::System::Xml::SingleArrayHelperWithDictionaryString {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadArray(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryReader>,
        localName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        namespaceUri: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryReader>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                        i32,
                        i32,
                    ), i32, 6usize>("ReadArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadArray",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (reader, localName, namespaceUri, array, offset, count),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryWriter>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        namespaceUri: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryWriter>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>("WriteArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteArray",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    writer,
                    prefix,
                    localName,
                    namespaceUri,
                    array,
                    offset,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Xml+SingleArrayHelperWithDictionaryString")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Xml::SingleArrayHelperWithDictionaryString
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
