#[cfg(feature = "System+ParameterizedStrings")]
#[repr(C)]
#[derive(Debug)]
pub struct ParameterizedStrings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ParameterizedStrings")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::ParameterizedStrings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ParameterizedStrings";
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
#[cfg(feature = "System+ParameterizedStrings")]
impl std::ops::Deref for crate::System::ParameterizedStrings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParameterizedStrings")]
impl std::ops::DerefMut for crate::System::ParameterizedStrings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParameterizedStrings")]
impl crate::System::ParameterizedStrings {
    #[cfg(feature = "System+ParameterizedStrings+FormatParam")]
    pub type FormatParam = crate::System::ParameterizedStrings_FormatParam;
    #[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
    pub type LowLevelStack = crate::System::ParameterizedStrings_LowLevelStack;
    pub fn AsBool(i: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsBool", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsInt(b: bool) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsInt", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Evaluate(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::ParameterizedStrings_FormatParam,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Evaluate", (format, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateInternal(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::ParameterizedStrings_FormatParam,
            >,
        >,
        stack: quest_hook::libil2cpp::Gc<
            crate::System::ParameterizedStrings_LowLevelStack,
        >,
        dynamicVars: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::System::ParameterizedStrings_FormatParam,
                >,
            >,
        >,
        staticVars: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::System::ParameterizedStrings_FormatParam,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EvaluateInternal",
                (format, pos, args, stack, dynamicVars, staticVars),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatPrintF(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatPrintF", (format, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDynamicOrStaticVariables(
        c: char,
        dynamicVars: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::System::ParameterizedStrings_FormatParam,
                >,
            >,
        >,
        staticVars: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::System::ParameterizedStrings_FormatParam,
                >,
            >,
        >,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::ParameterizedStrings_FormatParam,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::ParameterizedStrings_FormatParam,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDynamicOrStaticVariables", (c, dynamicVars, staticVars, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringFromAsciiBytes(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringFromAsciiBytes", (buffer, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn snprintf_Il2CppString0(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: crate::System::IntPtr,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("snprintf", (str, _cordl_size, format, arg1))?;
        Ok(__cordl_ret.into())
    }
    pub fn snprintf_i32_1(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: crate::System::IntPtr,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("snprintf", (str, _cordl_size, format, arg1))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ParameterizedStrings")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ParameterizedStrings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParameterizedStrings_FormatParam {
    pub _int32: i32,
    pub _string: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ParameterizedStrings_FormatParam {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ParameterizedStrings/FormatParam";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::ParameterizedStrings_FormatParam {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::ParameterizedStrings_FormatParam {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::ParameterizedStrings_FormatParam {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::ParameterizedStrings_FormatParam {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::ParameterizedStrings_FormatParam {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ParameterizedStrings+FormatParam")]
impl crate::System::ParameterizedStrings_FormatParam {
    pub fn _ctor_Il2CppString1(
        &mut self,
        intValue: i32,
        stringValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (intValue, stringValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Int32(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Int32",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_String(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_String", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ParameterizedStrings_FormatParam> {
        let __cordl_ret: crate::System::ParameterizedStrings_FormatParam = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
#[repr(C)]
#[derive(Debug)]
pub struct ParameterizedStrings_LowLevelStack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _arr: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::ParameterizedStrings_FormatParam,
        >,
    >,
    pub _count: i32,
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ParameterizedStrings_LowLevelStack {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ParameterizedStrings/LowLevelStack";
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
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
impl std::ops::Deref for crate::System::ParameterizedStrings_LowLevelStack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
impl std::ops::DerefMut for crate::System::ParameterizedStrings_LowLevelStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
impl crate::System::ParameterizedStrings_LowLevelStack {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Pop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ParameterizedStrings_FormatParam> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ParameterizedStrings_FormatParam = __cordl_object
            .invoke("Pop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Push(
        &mut self,
        item: crate::System::ParameterizedStrings_FormatParam,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ParameterizedStrings+LowLevelStack")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ParameterizedStrings_LowLevelStack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
