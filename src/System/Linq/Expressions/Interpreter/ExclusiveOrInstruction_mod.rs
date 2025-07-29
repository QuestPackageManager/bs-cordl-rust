#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+ExclusiveOrInstruction"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ExclusiveOrInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+ExclusiveOrInstruction"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "ExclusiveOrInstruction";
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
    feature = "cordl_class_System+Linq+Expressions+Interpreter+ExclusiveOrInstruction"
)]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_System+Linq+Expressions+Interpreter+ExclusiveOrInstruction"
)]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction")]
impl crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrBoolean"
    )]
    pub type ExclusiveOrBoolean = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrBoolean;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrByte"
    )]
    pub type ExclusiveOrByte = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrInt16"
    )]
    pub type ExclusiveOrInt16 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrInt32"
    )]
    pub type ExclusiveOrInt32 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrInt64"
    )]
    pub type ExclusiveOrInt64 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrSByte"
    )]
    pub type ExclusiveOrSByte = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrUInt16"
    )]
    pub type ExclusiveOrUInt16 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrUInt32"
    )]
    pub type ExclusiveOrUInt32 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrUInt64"
    )]
    pub type ExclusiveOrUInt64 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrUInt64;
    pub fn Create(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Interpreter::Instruction,
                        >,
                        1usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        > = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
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
    feature = "cordl_class_System+Linq+Expressions+Interpreter+ExclusiveOrInstruction"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
