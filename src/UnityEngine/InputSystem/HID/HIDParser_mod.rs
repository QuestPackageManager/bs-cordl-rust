#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser")]
#[repr(C)]
#[derive(Debug)]
pub struct HIDParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HIDParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDParser";
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::HID::HIDParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::HID::HIDParser {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
impl crate::UnityEngine::InputSystem::HID::HIDParser {
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
    pub type HIDItemStateGlobal = crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
    pub type HIDItemStateLocal = crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
    pub type HIDItemTypeAndTag = crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemTypeAndTag;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
    pub type HIDReportData = crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData;
    pub fn ParseReportDescriptor_Il2CppArray_ByRefMut0(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        deviceDescriptor: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ParseReportDescriptor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseReportDescriptor", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (buffer, deviceDescriptor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseReportDescriptor_Il2CppObject_i32_ByRefMut1(
        bufferPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLength: i32,
        deviceDescriptor: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ParseReportDescriptor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseReportDescriptor", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (bufferPtr, bufferLength, deviceDescriptor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadData(
        itemSize: i32,
        currentPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        endPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        i32,
                        3usize,
                    >("ReadData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadData", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (itemSize, currentPtr, endPtr))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::HID::HIDParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HIDParser_HIDItemStateGlobal {
    pub usagePage: crate::System::Nullable_1<i32>,
    pub logicalMinimum: crate::System::Nullable_1<i32>,
    pub logicalMaximum: crate::System::Nullable_1<i32>,
    pub physicalMinimum: crate::System::Nullable_1<i32>,
    pub physicalMaximum: crate::System::Nullable_1<i32>,
    pub unitExponent: crate::System::Nullable_1<i32>,
    pub unit: crate::System::Nullable_1<i32>,
    pub reportSize: crate::System::Nullable_1<i32>,
    pub reportCount: crate::System::Nullable_1<i32>,
    pub reportId: crate::System::Nullable_1<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDParser/HIDItemStateGlobal";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
impl crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
    pub fn GetPhysicalMax(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetPhysicalMax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicalMax", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicalMin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetPhysicalMin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicalMin", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUsagePage(
        &mut self,
        index: i32,
        localItemState: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::HID::HID_UsagePage,
                        2usize,
                    >("GetUsagePage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUsagePage", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_UsagePage = unsafe {
            cordl_method_info.invoke_unchecked(self, (index, localItemState))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HIDParser_HIDItemStateLocal {
    pub usage: crate::System::Nullable_1<i32>,
    pub usageMinimum: crate::System::Nullable_1<i32>,
    pub usageMaximum: crate::System::Nullable_1<i32>,
    pub designatorIndex: crate::System::Nullable_1<i32>,
    pub designatorMinimum: crate::System::Nullable_1<i32>,
    pub designatorMaximum: crate::System::Nullable_1<i32>,
    pub stringIndex: crate::System::Nullable_1<i32>,
    pub stringMinimum: crate::System::Nullable_1<i32>,
    pub stringMaximum: crate::System::Nullable_1<i32>,
    pub usageList: quest_hook::libil2cpp::Gc<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDParser/HIDItemStateLocal";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
impl crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
    pub fn GetUsage(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("GetUsage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUsage", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        state: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetUsage(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetUsage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetUsage", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HIDParser_HIDItemTypeAndTag {
    #[default]
    Collection = 160i32,
    Delimiter = 168i32,
    DesignatorIndex = 56i32,
    DesignatorMaximum = 88i32,
    DesignatorMinimum = 72i32,
    EndCollection = 192i32,
    Feature = 176i32,
    Input = 128i32,
    LogicalMaximum = 36i32,
    LogicalMinimum = 20i32,
    Output = 144i32,
    PhysicalMaximum = 68i32,
    PhysicalMinimum = 52i32,
    Pop = 180i32,
    Push = 164i32,
    ReportCount = 148i32,
    ReportID = 132i32,
    ReportSize = 116i32,
    StringIndex = 120i32,
    StringMaximum = 152i32,
    StringMinimum = 136i32,
    Unit = 100i32,
    UnitExponent = 84i32,
    Usage = 8i32,
    UsageMaximum = 40i32,
    UsageMinimum = 24i32,
    UsagePage = 4i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemTypeAndTag {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDParser/HIDItemTypeAndTag";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemTypeAndTag {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemTypeAndTag {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemTypeAndTag {
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemTypeAndTag {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HIDParser_HIDReportData {
    pub reportId: i32,
    pub reportType: crate::UnityEngine::InputSystem::HID::HID_HIDReportType,
    pub currentBitOffset: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDParser/HIDReportData";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
impl crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {
    pub fn FindOrAddReport(
        reportId: crate::System::Nullable_1<i32>,
        reportType: crate::UnityEngine::InputSystem::HID::HID_HIDReportType,
        reports: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::Nullable_1<i32>,
                            crate::UnityEngine::InputSystem::HID::HID_HIDReportType,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData,
                            >,
                        ),
                        i32,
                        3usize,
                    >("FindOrAddReport")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindOrAddReport", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (reportId, reportType, reports))?
        };
        Ok(__cordl_ret.into())
    }
}
