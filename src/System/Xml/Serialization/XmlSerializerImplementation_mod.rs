#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializerImplementation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Serialization::XmlSerializerImplementation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Serialization";
    const CLASS_NAME: &'static str = "XmlSerializerImplementation";
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
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSerializerImplementation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializerImplementation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
impl crate::System::Xml::Serialization::XmlSerializerImplementation {
    pub fn get_Writer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Serialization::XmlSerializationWriter,
                >,
                0usize,
            >("get_Writer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Writer", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriter,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializerImplementation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
