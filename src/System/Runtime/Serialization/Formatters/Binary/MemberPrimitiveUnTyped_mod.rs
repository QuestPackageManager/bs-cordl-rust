#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberPrimitiveUnTyped {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub typeInformation: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "MemberPrimitiveUnTyped";
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
impl crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
    pub fn Dump(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dump")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dump", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        input: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Read", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Set_Il2CppObject0(
        &mut self,
        typeInformation: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Set")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Set", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeInformation, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Set_InternalPrimitiveTypeE1(
        &mut self,
        typeInformation: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Set")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Set", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeInformation))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        sout: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Write", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sout))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
