#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlRawWriterBase64Encoder {
    __cordl_parent: crate::System::Xml::Base64Encoder,
    pub rawWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter>,
}
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::XmlRawWriterBase64Encoder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlRawWriterBase64Encoder";
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
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
impl std::ops::Deref for crate::System::Xml::XmlRawWriterBase64Encoder {
    type Target = crate::System::Xml::Base64Encoder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
impl std::ops::DerefMut for crate::System::Xml::XmlRawWriterBase64Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
impl crate::System::Xml::XmlRawWriterBase64Encoder {
    pub fn New(
        rawWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rawWriter))?;
        Ok(__cordl_object.into())
    }
    pub fn WriteChars(
        &mut self,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteChars", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (chars, index, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rawWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlRawWriter>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rawWriter))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlRawWriterBase64Encoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
