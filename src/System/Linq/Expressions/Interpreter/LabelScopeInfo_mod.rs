#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct LabelScopeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _labels: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::HybridReferenceDictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LabelInfo,
            >,
        >,
    >,
    pub Kind: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    pub Parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "LabelScopeInfo";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
impl crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    pub fn AddLabelInfo(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::LabelTarget,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Interpreter::LabelInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddLabelInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddLabelInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (target, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsTarget(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::LabelTarget,
                >),
                bool,
                1usize,
            >("ContainsTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ContainsTarget", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (target)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
        kind: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent, kind))?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetLabelInfo(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        info: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LabelInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::LabelTarget,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Interpreter::LabelInfo,
                        >,
                    >,
                ),
                bool,
                2usize,
            >("TryGetLabelInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetLabelInfo", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (target, info)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
        kind: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
                    >,
                    crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parent, kind))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CanJumpInto(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_CanJumpInto")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_CanJumpInto", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
