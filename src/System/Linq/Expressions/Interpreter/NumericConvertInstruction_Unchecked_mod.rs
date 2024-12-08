#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+Unchecked"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NumericConvertInstruction_Unchecked {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::NumericConvertInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+Unchecked"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NumericConvertInstruction_Unchecked =>
    "System.Linq.Expressions.Interpreter"."NumericConvertInstruction/Unchecked"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+Unchecked"
)]
impl std::ops::Deref for crate::GlobalNamespace::NumericConvertInstruction_Unchecked {
    type Target = crate::System::Linq::Expressions::Interpreter::NumericConvertInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+Unchecked"
)]
impl std::ops::DerefMut for crate::GlobalNamespace::NumericConvertInstruction_Unchecked {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+Unchecked"
)]
impl crate::GlobalNamespace::NumericConvertInstruction_Unchecked {
    pub fn _ctor(
        &mut self,
        from: crate::System::TypeCode,
        to: crate::System::TypeCode,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (from, to, isLiftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn Convert(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Convert", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertDouble(
        &mut self,
        obj: f64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertDouble", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertInt32(
        &mut self,
        obj: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertInt32", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_InstructionName", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConvertUInt64(
        &mut self,
        obj: u64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertUInt64", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertInt64(
        &mut self,
        obj: i64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertInt64", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        from: crate::System::TypeCode,
        to: crate::System::TypeCode,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (from, to, isLiftedToNull))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+Unchecked"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NumericConvertInstruction_Unchecked {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
