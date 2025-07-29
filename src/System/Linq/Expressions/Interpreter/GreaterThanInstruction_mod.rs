#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanInstruction"
)]
#[repr(C)]
#[derive(Debug)]
pub struct GreaterThanInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanInstruction"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "GreaterThanInstruction";
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
#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanInstruction"
)]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanInstruction"
)]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction")]
impl crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanByte"
    )]
    pub type GreaterThanByte = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanChar"
    )]
    pub type GreaterThanChar = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanChar;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanDouble"
    )]
    pub type GreaterThanDouble = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanDouble;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanInt16"
    )]
    pub type GreaterThanInt16 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanInt32"
    )]
    pub type GreaterThanInt32 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanInt64"
    )]
    pub type GreaterThanInt64 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSByte"
    )]
    pub type GreaterThanSByte = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSingle"
    )]
    pub type GreaterThanSingle = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanUInt16"
    )]
    pub type GreaterThanUInt16 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanUInt32"
    )]
    pub type GreaterThanUInt32 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanUInt64"
    )]
    pub type GreaterThanUInt64 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanUInt64;
    pub fn Create(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Interpreter::Instruction,
                        >,
                        2usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, liftedToNull))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nullValue))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
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
            cordl_method_info.invoke_unchecked(self, (nullValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ConsumedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_ConsumedStack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ConsumedStack", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_InstructionName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_InstructionName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_ProducedStack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ProducedStack", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanInstruction"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
