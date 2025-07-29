#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction"
)]
#[repr(C)]
#[derive(Debug)]
pub struct GreaterThanOrEqualInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "GreaterThanOrEqualInstruction";
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
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction"
)]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction"
)]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction")]
impl crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualByte"
    )]
    pub type GreaterThanOrEqualByte = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualChar"
    )]
    pub type GreaterThanOrEqualChar = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualChar;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualDouble"
    )]
    pub type GreaterThanOrEqualDouble = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualDouble;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualInt16"
    )]
    pub type GreaterThanOrEqualInt16 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualInt32"
    )]
    pub type GreaterThanOrEqualInt32 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualInt64"
    )]
    pub type GreaterThanOrEqualInt64 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSByte"
    )]
    pub type GreaterThanOrEqualSByte = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSingle"
    )]
    pub type GreaterThanOrEqualSingle = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualUInt16"
    )]
    pub type GreaterThanOrEqualUInt16 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualUInt32"
    )]
    pub type GreaterThanOrEqualUInt32 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualUInt64"
    )]
    pub type GreaterThanOrEqualUInt64 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualUInt64;
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
    feature = "cordl_class_System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
