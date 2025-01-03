#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+IOUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct IOUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+IOUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::IOUtil =>
    "System.Runtime.Serialization.Formatters.Binary"."IOUtil"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+IOUtil")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::IOUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+IOUtil")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::IOUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+IOUtil")]
impl crate::System::Runtime::Serialization::Formatters::Binary::IOUtil {
    pub fn FlagTest(
        flag: crate::System::Runtime::Serialization::Formatters::Binary::MessageEnum,
        target: crate::System::Runtime::Serialization::Formatters::Binary::MessageEnum,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FlagTest", (flag, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStringWithCode(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sout: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteStringWithCode", (value, sout))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteWithCode(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        sout: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteWithCode", (_cordl_type, value, sout))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+IOUtil")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::IOUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
