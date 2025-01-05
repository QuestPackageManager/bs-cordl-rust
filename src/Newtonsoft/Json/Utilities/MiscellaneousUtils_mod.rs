#[cfg(feature = "Newtonsoft+Json+Utilities+MiscellaneousUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct MiscellaneousUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MiscellaneousUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::MiscellaneousUtils
    => "Newtonsoft.Json.Utilities"."MiscellaneousUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+MiscellaneousUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::MiscellaneousUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MiscellaneousUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::MiscellaneousUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MiscellaneousUtils")]
impl crate::Newtonsoft::Json::Utilities::MiscellaneousUtils {
    pub fn ByteArrayCompare(
        a1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        a2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ByteArrayCompare", (a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgumentOutOfRangeException(
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        actualValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateArgumentOutOfRangeException",
                (paramName, actualValue, message),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalName(
        qualifiedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalName", (qualifiedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefix(
        qualifiedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrefix", (qualifiedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQualifiedNameParts(
        qualifiedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prefix: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        localName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQualifiedNameParts", (qualifiedName, prefix, localName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRegexOptions(
        optionsText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Text::RegularExpressions::RegexOptions,
    > {
        let __cordl_ret: crate::System::Text::RegularExpressions::RegexOptions = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRegexOptions", (optionsText))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValueEquals(
        objA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objB: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValueEquals", (objA, objB))?;
        Ok(__cordl_ret.into())
    }
    pub fn _cordl_Assert(
        condition: bool,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Assert", (condition, message))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MiscellaneousUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::MiscellaneousUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
