#[cfg(feature = "System+RuntimeTypeHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RuntimeTypeHandle {
    pub value: crate::System::IntPtr,
}
#[cfg(feature = "System+RuntimeTypeHandle")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::RuntimeTypeHandle {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "RuntimeTypeHandle";
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
#[cfg(feature = "System+RuntimeTypeHandle")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::RuntimeTypeHandle {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+RuntimeTypeHandle")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::RuntimeTypeHandle {
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
#[cfg(feature = "System+RuntimeTypeHandle")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::RuntimeTypeHandle {
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
#[cfg(feature = "System+RuntimeTypeHandle")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::RuntimeTypeHandle {
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
#[cfg(feature = "System+RuntimeTypeHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::RuntimeTypeHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+RuntimeTypeHandle")]
impl crate::System::RuntimeTypeHandle {
    pub fn CanCastTo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        target: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanCastTo", (_cordl_type, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayRank(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArrayRank", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssembly(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeAssembly>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::RuntimeAssembly,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssembly", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::TypeAttributes> {
        let __cordl_ret: crate::System::Reflection::TypeAttributes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::RuntimeType> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBaseType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCorElementType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::CorElementType> {
        let __cordl_ret: crate::System::Reflection::CorElementType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCorElementType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetElementType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::RuntimeType> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetElementType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGenericParameterInfo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGenericParameterInfo", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGenericTypeDefinition(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGenericTypeDefinition", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGenericTypeDefinition_impl(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGenericTypeDefinition_impl", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMetadataToken(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMetadataToken", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetModule(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeModule>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::RuntimeModule,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetModule", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetObjectData",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetToken(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetToken", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeByName(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        throwOnError: bool,
        ignoreCase: bool,
        reflectionOnly: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
        loadTypeFromPartialName: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::RuntimeType> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetTypeByName",
                (
                    typeName,
                    throwOnError,
                    ignoreCase,
                    reflectionOnly,
                    stackMark,
                    loadTypeFromPartialName,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HasElementType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasElementType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasInstantiation(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasInstantiation", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasReferences(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasReferences", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsArray(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsArray", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsByRef(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsByRef", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsComObject_RuntimeType0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsComObject", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsComObject__cordl_bool1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        isGenericCOM: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsComObject", (_cordl_type, isGenericCOM))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsContextful(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsContextful", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        rtType1: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        rtType2: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEquivalentTo", (rtType1, rtType2))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericTypeDefinition(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenericTypeDefinition", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericVariable(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenericVariable", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInstanceOfType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInstanceOfType", (_cordl_type, o))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInterface(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInterface", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPointer(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPointer", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrimitive(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrimitive", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSubclassOf(
        childType: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        baseType: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSubclassOf", (childType, baseType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSzArray(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSzArray", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IntPtr0(
        &mut self,
        val: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (val),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RuntimeType1(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn internal_from_name(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
        callerAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        throwOnError: bool,
        ignoreCase: bool,
        reflectionOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::RuntimeType> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "internal_from_name",
                (
                    name,
                    stackMark,
                    callerAssembly,
                    throwOnError,
                    ignoreCase,
                    reflectionOnly,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn is_subclass_of(
        childType: crate::System::IntPtr,
        baseType: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("is_subclass_of", (childType, baseType))?;
        Ok(__cordl_ret.into())
    }
    pub fn type_is_assignable_from(
        a: quest_hook::libil2cpp::Gc<crate::System::Type>,
        b: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("type_is_assignable_from", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+RuntimeTypeHandle")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::RuntimeTypeHandle {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
#[cfg(feature = "System+RuntimeTypeHandle")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::RuntimeTypeHandle {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
