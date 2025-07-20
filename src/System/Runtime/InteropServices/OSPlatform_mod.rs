#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OSPlatform {
    pub _osPlatform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::OSPlatform {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "OSPlatform";
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
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Runtime::InteropServices::OSPlatform {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Runtime::InteropServices::OSPlatform {
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
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Runtime::InteropServices::OSPlatform {
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
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Runtime::InteropServices::OSPlatform {
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
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::InteropServices::OSPlatform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
impl crate::System::Runtime::InteropServices::OSPlatform {
    pub fn Create(
        osPlatform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::Runtime::InteropServices::OSPlatform,
                1usize,
            >("Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "Create", 1usize
                )
            });
        let __cordl_ret: crate::System::Runtime::InteropServices::OSPlatform = unsafe {
            method.invoke_unchecked((), (osPlatform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject2(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppString1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OSPlatform0(
        &mut self,
        other: crate::System::Runtime::InteropServices::OSPlatform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Runtime::InteropServices::OSPlatform),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        osPlatform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (osPlatform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Linux() -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::Runtime::InteropServices::OSPlatform,
                0usize,
            >("get_Linux")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "get_Linux", 0usize
                )
            });
        let __cordl_ret: crate::System::Runtime::InteropServices::OSPlatform = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_OSX() -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::Runtime::InteropServices::OSPlatform,
                0usize,
            >("get_OSX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "get_OSX", 0usize
                )
            });
        let __cordl_ret: crate::System::Runtime::InteropServices::OSPlatform = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Windows() -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::Runtime::InteropServices::OSPlatform,
                0usize,
            >("get_Windows")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "get_Windows", 0usize
                )
            });
        let __cordl_ret: crate::System::Runtime::InteropServices::OSPlatform = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::System::Runtime::InteropServices::OSPlatform,
        right: crate::System::Runtime::InteropServices::OSPlatform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::OSPlatform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Runtime::InteropServices::OSPlatform,
                    crate::System::Runtime::InteropServices::OSPlatform,
                ),
                bool,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::OSPlatform as
                    quest_hook::libil2cpp::Type > ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (left, right))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
impl AsRef<
    crate::System::IEquatable_1<crate::System::Runtime::InteropServices::OSPlatform>,
> for crate::System::Runtime::InteropServices::OSPlatform {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+InteropServices+OSPlatform")]
impl AsMut<
    crate::System::IEquatable_1<crate::System::Runtime::InteropServices::OSPlatform>,
> for crate::System::Runtime::InteropServices::OSPlatform {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::System::Runtime::InteropServices::OSPlatform,
    > {
        todo!()
    }
}
