#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugInfo {
    __cordl_parent: crate::System::Object,
    pub StartLine: i32,
    pub EndLine: i32,
    pub Index: i32,
    pub FileName: *mut crate::System::String,
    pub IsClear: bool,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::DebugInfo =>
    "System.Linq.Expressions.Interpreter"."DebugInfo"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::DebugInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Interpreter::DebugInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo")]
impl crate::System::Linq::Expressions::Interpreter::DebugInfo {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo+DebugInfoComparer")]
    pub type DebugInfoComparer = crate::System::Linq::Expressions::Interpreter::DebugInfo_DebugInfoComparer;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::DebugInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo+DebugInfoComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugInfo_DebugInfoComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo+DebugInfoComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::DebugInfo_DebugInfoComparer =>
    "System.Linq.Expressions.Interpreter"."DebugInfo/DebugInfoComparer"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo+DebugInfoComparer")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::DebugInfo_DebugInfoComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo+DebugInfoComparer")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::DebugInfo_DebugInfoComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo+DebugInfoComparer")]
impl crate::System::Linq::Expressions::Interpreter::DebugInfo_DebugInfoComparer {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn System_Collections_Generic_IComparer_System_Linq_Expressions_Interpreter_DebugInfo__Compare(
        &mut self,
        d1: *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
        d2: *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.Collections.Generic.IComparer<System.Linq.Expressions.Interpreter.DebugInfo>.Compare",
                (d1, d2),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DebugInfo+DebugInfoComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::DebugInfo_DebugInfoComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
