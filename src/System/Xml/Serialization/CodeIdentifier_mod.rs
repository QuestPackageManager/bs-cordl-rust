#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
#[repr(C)]
#[derive(Debug)]
pub struct CodeIdentifier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Serialization::CodeIdentifier {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Serialization";
    const CLASS_NAME: &'static str = "CodeIdentifier";
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
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
impl std::ops::Deref for crate::System::Xml::Serialization::CodeIdentifier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::CodeIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
impl crate::System::Xml::Serialization::CodeIdentifier {
    pub fn IsValid(c: char) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), bool, 1usize>("IsValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsValid", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (c)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidStart(c: char) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), bool, 1usize>("IsValidStart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsValidStart", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (c)) };
        Ok(__cordl_ret.into())
    }
    pub fn MakePascal(
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("MakePascal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MakePascal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (identifier)) };
        Ok(__cordl_ret.into())
    }
    pub fn MakeValid(
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("MakeValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MakeValid", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (identifier)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::CodeIdentifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
