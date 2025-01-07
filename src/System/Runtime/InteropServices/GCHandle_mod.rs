#[cfg(feature = "System+Runtime+InteropServices+GCHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GCHandle {
    pub handle: crate::System::IntPtr,
}
#[cfg(feature = "System+Runtime+InteropServices+GCHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::GCHandle {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "GCHandle";
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
#[cfg(feature = "System+Runtime+InteropServices+GCHandle")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Runtime::InteropServices::GCHandle {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Runtime+InteropServices+GCHandle")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Runtime::InteropServices::GCHandle {
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
#[cfg(feature = "System+Runtime+InteropServices+GCHandle")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Runtime::InteropServices::GCHandle {
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
#[cfg(feature = "System+Runtime+InteropServices+GCHandle")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Runtime::InteropServices::GCHandle {
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
#[cfg(feature = "System+Runtime+InteropServices+GCHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::InteropServices::GCHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+GCHandle")]
impl crate::System::Runtime::InteropServices::GCHandle {
    pub fn AddrOfPinnedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddrOfPinnedObject",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_GCHandleType1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: crate::System::Runtime::InteropServices::GCHandleType,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::GCHandle,
    > {
        let __cordl_ret: crate::System::Runtime::InteropServices::GCHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Alloc", (value, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::GCHandle,
    > {
        let __cordl_ret: crate::System::Runtime::InteropServices::GCHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Alloc", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCurrentDomain(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckCurrentDomain", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Free(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Free",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeHandle(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeHandle", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromIntPtr(
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::GCHandle,
    > {
        let __cordl_ret: crate::System::Runtime::InteropServices::GCHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromIntPtr", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAddrOfPinnedObject(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAddrOfPinnedObject", (handle))?;
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
    pub fn GetTarget(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTarget", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTargetHandle(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        handle: crate::System::IntPtr,
        _cordl_type: crate::System::Runtime::InteropServices::GCHandleType,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTargetHandle", (obj, handle, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToIntPtr(
        value: crate::System::Runtime::InteropServices::GCHandle,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToIntPtr", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_GCHandleType2(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: crate::System::Runtime::InteropServices::GCHandleType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, _cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IntPtr0(
        &mut self,
        h: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (h),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAllocated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsAllocated",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Target(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Target", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::System::Runtime::InteropServices::GCHandle,
        b: crate::System::Runtime::InteropServices::GCHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_GCHandle0(
        value: crate::System::Runtime::InteropServices::GCHandle,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_IntPtr1(
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::GCHandle,
    > {
        let __cordl_ret: crate::System::Runtime::InteropServices::GCHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Target(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Target",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
