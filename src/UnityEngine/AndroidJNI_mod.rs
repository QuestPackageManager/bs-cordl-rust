#[cfg(feature = "UnityEngine+AndroidJNI")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJNI {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AndroidJNI {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AndroidJNI";
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
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl std::ops::Deref for crate::UnityEngine::AndroidJNI {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJNI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl crate::UnityEngine::AndroidJNI {
    pub fn AllocObject(
        clazz: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("AllocObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AllocObject", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AttachCurrentThread() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("AttachCurrentThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AttachCurrentThread", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn CallBooleanMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                bool,
                3usize,
            >("CallBooleanMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallBooleanMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallBooleanMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                bool,
                3usize,
            >("CallBooleanMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallBooleanMethod", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallBooleanMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                bool,
                3usize,
            >("CallBooleanMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallBooleanMethod", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallByteMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                u8,
                3usize,
            >("CallByteMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallByteMethod", 3usize
                )
            });
        let __cordl_ret: u8 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallCharMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                char,
                3usize,
            >("CallCharMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallCharMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallCharMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                char,
                3usize,
            >("CallCharMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallCharMethod", 3usize
                )
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallCharMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                char,
                3usize,
            >("CallCharMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallCharMethod", 3usize
                )
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallDoubleMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                f64,
                3usize,
            >("CallDoubleMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallDoubleMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallDoubleMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                f64,
                3usize,
            >("CallDoubleMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallDoubleMethod", 3usize
                )
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallDoubleMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                f64,
                3usize,
            >("CallDoubleMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallDoubleMethod", 3usize
                )
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallFloatMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                f32,
                3usize,
            >("CallFloatMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallFloatMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallFloatMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                f32,
                3usize,
            >("CallFloatMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallFloatMethod", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallFloatMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                f32,
                3usize,
            >("CallFloatMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallFloatMethod", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallIntMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i32,
                3usize,
            >("CallIntMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallIntMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallIntMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                i32,
                3usize,
            >("CallIntMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallIntMethod", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallIntMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                i32,
                3usize,
            >("CallIntMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallIntMethod", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallLongMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i64,
                3usize,
            >("CallLongMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallLongMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallLongMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                i64,
                3usize,
            >("CallLongMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallLongMethod", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallLongMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                i64,
                3usize,
            >("CallLongMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallLongMethod", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallObjectMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                crate::System::IntPtr,
                3usize,
            >("CallObjectMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallObjectMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallObjectMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                crate::System::IntPtr,
                3usize,
            >("CallObjectMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallObjectMethod", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallObjectMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                crate::System::IntPtr,
                3usize,
            >("CallObjectMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallObjectMethod", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallSByteMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i8,
                3usize,
            >("CallSByteMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallSByteMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: i8 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallSByteMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                i8,
                3usize,
            >("CallSByteMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallSByteMethod", 3usize
                )
            });
        let __cordl_ret: i8 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallSByteMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                i8,
                3usize,
            >("CallSByteMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallSByteMethod", 3usize
                )
            });
        let __cordl_ret: i8 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallShortMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i16,
                3usize,
            >("CallShortMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallShortMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: i16 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallShortMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                i16,
                3usize,
            >("CallShortMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallShortMethod", 3usize
                )
            });
        let __cordl_ret: i16 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallShortMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                i16,
                3usize,
            >("CallShortMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallShortMethod", 3usize
                )
            });
        let __cordl_ret: i16 = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticBooleanMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                bool,
                3usize,
            >("CallStaticBooleanMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticBooleanMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticBooleanMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                bool,
                3usize,
            >("CallStaticBooleanMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticBooleanMethod", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticBooleanMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                bool,
                3usize,
            >("CallStaticBooleanMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticBooleanMethod", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticByteMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                u8,
                3usize,
            >("CallStaticByteMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticByteMethod", 3usize
                )
            });
        let __cordl_ret: u8 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticCharMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                char,
                3usize,
            >("CallStaticCharMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticCharMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticCharMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                char,
                3usize,
            >("CallStaticCharMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticCharMethod", 3usize
                )
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticCharMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                char,
                3usize,
            >("CallStaticCharMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticCharMethod", 3usize
                )
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticDoubleMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                f64,
                3usize,
            >("CallStaticDoubleMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticDoubleMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticDoubleMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                f64,
                3usize,
            >("CallStaticDoubleMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticDoubleMethod", 3usize
                )
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticDoubleMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                f64,
                3usize,
            >("CallStaticDoubleMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticDoubleMethod", 3usize
                )
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticFloatMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                f32,
                3usize,
            >("CallStaticFloatMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticFloatMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticFloatMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                f32,
                3usize,
            >("CallStaticFloatMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticFloatMethod", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticFloatMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                f32,
                3usize,
            >("CallStaticFloatMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticFloatMethod", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticIntMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i32,
                3usize,
            >("CallStaticIntMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticIntMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticIntMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                i32,
                3usize,
            >("CallStaticIntMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticIntMethod", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticIntMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                i32,
                3usize,
            >("CallStaticIntMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticIntMethod", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticLongMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i64,
                3usize,
            >("CallStaticLongMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticLongMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticLongMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                i64,
                3usize,
            >("CallStaticLongMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticLongMethod", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticLongMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                i64,
                3usize,
            >("CallStaticLongMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticLongMethod", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticObjectMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                crate::System::IntPtr,
                3usize,
            >("CallStaticObjectMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticObjectMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticObjectMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                crate::System::IntPtr,
                3usize,
            >("CallStaticObjectMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticObjectMethod", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticObjectMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                crate::System::IntPtr,
                3usize,
            >("CallStaticObjectMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticObjectMethod", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticSByteMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i8,
                3usize,
            >("CallStaticSByteMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticSByteMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: i8 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticSByteMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                i8,
                3usize,
            >("CallStaticSByteMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticSByteMethod", 3usize
                )
            });
        let __cordl_ret: i8 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticSByteMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                i8,
                3usize,
            >("CallStaticSByteMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticSByteMethod", 3usize
                )
            });
        let __cordl_ret: i8 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticShortMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i16,
                3usize,
            >("CallStaticShortMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticShortMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: i16 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticShortMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                i16,
                3usize,
            >("CallStaticShortMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticShortMethod", 3usize
                )
            });
        let __cordl_ret: i16 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticShortMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                i16,
                3usize,
            >("CallStaticShortMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticShortMethod", 3usize
                )
            });
        let __cordl_ret: i16 = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticStringMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("CallStaticStringMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticStringMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (clazz, methodID, args)) };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticStringMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("CallStaticStringMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticStringMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (clazz, methodID, args)) };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticStringMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("CallStaticStringMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticStringMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (clazz, methodID, args)) };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticVoidMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CallStaticVoidMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticVoidMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticVoidMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CallStaticVoidMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticVoidMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticVoidMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CallStaticVoidMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStaticVoidMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallStringMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("CallStringMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStringMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj, methodID, args)) };
        Ok(__cordl_ret.into())
    }
    pub fn CallStringMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("CallStringMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStringMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj, methodID, args)) };
        Ok(__cordl_ret.into())
    }
    pub fn CallStringMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("CallStringMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallStringMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj, methodID, args)) };
        Ok(__cordl_ret.into())
    }
    pub fn CallVoidMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CallVoidMethodUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallVoidMethodUnsafe", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallVoidMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CallVoidMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallVoidMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallVoidMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CallVoidMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CallVoidMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DeleteGlobalRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeleteGlobalRef", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteLocalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DeleteLocalRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeleteLocalRef", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteWeakGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DeleteWeakGlobalRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeleteWeakGlobalRef", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DetachCurrentThread() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("DetachCurrentThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DetachCurrentThread", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureLocalCapacity(capacity: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("EnsureLocalCapacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureLocalCapacity", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (capacity)) };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptionClear() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ExceptionClear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ExceptionClear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptionDescribe() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ExceptionDescribe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ExceptionDescribe", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptionOccurred() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::System::IntPtr, 0usize>("ExceptionOccurred")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ExceptionOccurred", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn FatalError(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FatalError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FatalError", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindClass(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::IntPtr,
                1usize,
            >("FindClass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindClass", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (name))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromBooleanArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
                1usize,
            >("FromBooleanArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromBooleanArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromByteArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("FromByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromByteArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromCharArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                1usize,
            >("FromCharArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromCharArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromDoubleArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>,
                1usize,
            >("FromDoubleArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromDoubleArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f64>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromFloatArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                1usize,
            >("FromFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromFloatArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromIntArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                1usize,
            >("FromIntArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromIntArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromLongArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
                1usize,
            >("FromLongArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromLongArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i64>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromObjectArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
                >,
                1usize,
            >("FromObjectArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromObjectArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromReflectedField(
        refField: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("FromReflectedField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromReflectedField", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (refField))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromReflectedMethod(
        refMethod: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("FromReflectedMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromReflectedMethod", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (refMethod))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromSByteArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
                1usize,
            >("FromSByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromSByteArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i8>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromShortArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                1usize,
            >("FromShortArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromShortArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i16>,
        > = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayLength(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::IntPtr), i32, 1usize>("GetArrayLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetArrayLength", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetBooleanArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                bool,
                2usize,
            >("GetBooleanArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBooleanArrayElement", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetBooleanField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                bool,
                2usize,
            >("GetBooleanField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBooleanField", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetByteArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                u8,
                2usize,
            >("GetByteArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetByteArrayElement", 2usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                u8,
                2usize,
            >("GetByteField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetByteField", 2usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCharArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                char,
                2usize,
            >("GetCharArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCharArrayElement", 2usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCharField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                char,
                2usize,
            >("GetCharField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCharField", 2usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectBuffer<T>(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Unity::Collections::NativeArray_1<T>,
                1usize,
            >("GetDirectBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDirectBuffer", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = unsafe {
            method.invoke_unchecked((), (buffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectBufferAddress(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("GetDirectBufferAddress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDirectBufferAddress", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (buffer)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectBufferCapacity(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("GetDirectBufferCapacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDirectBufferCapacity", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (buffer)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectByteBuffer(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Unity::Collections::NativeArray_1<u8>,
                1usize,
            >("GetDirectByteBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDirectByteBuffer", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = unsafe {
            method.invoke_unchecked((), (buffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectSByteBuffer(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<i8>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Unity::Collections::NativeArray_1<i8>,
                1usize,
            >("GetDirectSByteBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDirectSByteBuffer", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<i8> = unsafe {
            method.invoke_unchecked((), (buffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDoubleArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                f64,
                2usize,
            >("GetDoubleArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDoubleArrayElement", 2usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDoubleField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                f64,
                2usize,
            >("GetDoubleField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDoubleField", 2usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldID(
        clazz: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::System::IntPtr,
                3usize,
            >("GetFieldID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFieldID", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, name, sig))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFloatArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                f32,
                2usize,
            >("GetFloatArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFloatArrayElement", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetFloatField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                f32,
                2usize,
            >("GetFloatField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFloatField", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetIntArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                i32,
                2usize,
            >("GetIntArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetIntArrayElement", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetIntField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                i32,
                2usize,
            >("GetIntField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetIntField", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetJavaVM() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::System::IntPtr, 0usize>("GetJavaVM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetJavaVM", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLongArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                i64,
                2usize,
            >("GetLongArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLongArrayElement", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLongField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                i64,
                2usize,
            >("GetLongField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLongField", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID(
        clazz: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::System::IntPtr,
                3usize,
            >("GetMethodID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMethodID", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, name, sig))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                crate::System::IntPtr,
                2usize,
            >("GetObjectArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetObjectArrayElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectClass(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("GetObjectClass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetObjectClass", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                crate::System::IntPtr,
                2usize,
            >("GetObjectField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetObjectField", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, fieldID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetQueueGlobalRefsCount() -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u32, 0usize>("GetQueueGlobalRefsCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetQueueGlobalRefsCount", 0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSByteArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                i8,
                2usize,
            >("GetSByteArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSByteArrayElement", 2usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                i8,
                2usize,
            >("GetSByteField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSByteField", 2usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetShortArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                i16,
                2usize,
            >("GetShortArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetShortArrayElement", 2usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (array, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetShortField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                i16,
                2usize,
            >("GetShortField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetShortField", 2usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticBooleanField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                bool,
                2usize,
            >("GetStaticBooleanField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticBooleanField", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticByteField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                u8,
                2usize,
            >("GetStaticByteField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticByteField", 2usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticCharField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                char,
                2usize,
            >("GetStaticCharField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticCharField", 2usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticDoubleField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                f64,
                2usize,
            >("GetStaticDoubleField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticDoubleField", 2usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticFieldID(
        clazz: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::System::IntPtr,
                3usize,
            >("GetStaticFieldID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticFieldID", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, name, sig))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticFloatField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                f32,
                2usize,
            >("GetStaticFloatField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticFloatField", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticIntField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                i32,
                2usize,
            >("GetStaticIntField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticIntField", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticLongField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                i64,
                2usize,
            >("GetStaticLongField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticLongField", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticMethodID(
        clazz: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::System::IntPtr,
                3usize,
            >("GetStaticMethodID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticMethodID", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, name, sig))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticObjectField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                crate::System::IntPtr,
                2usize,
            >("GetStaticObjectField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticObjectField", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, fieldID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticSByteField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                i8,
                2usize,
            >("GetStaticSByteField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticSByteField", 2usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticShortField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                i16,
                2usize,
            >("GetStaticShortField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticShortField", 2usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticStringField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetStaticStringField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStaticStringField", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (clazz, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringChars(
        str: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetStringChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStringChars", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (str)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetStringField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStringField", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj, fieldID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringLength(
        str: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("GetStringLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStringLength", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (str)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringUTFChars(
        str: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetStringUTFChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStringUTFChars", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (str)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringUTFLength(
        str: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("GetStringUTFLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStringUTFLength", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (str)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSuperclass(
        clazz: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("GetSuperclass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSuperclass", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetVersion() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("GetVersion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetVersion", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IsAssignableFrom(
        clazz1: crate::System::IntPtr,
        clazz2: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                bool,
                2usize,
            >("IsAssignableFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsAssignableFrom", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (clazz1, clazz2)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsInstanceOf(
        obj: crate::System::IntPtr,
        clazz: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                bool,
                2usize,
            >("IsInstanceOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsInstanceOf", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj, clazz)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsSameObject(
        obj1: crate::System::IntPtr,
        obj2: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                bool,
                2usize,
            >("IsSameObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsSameObject", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj1, obj2)) };
        Ok(__cordl_ret.into())
    }
    pub fn NewBooleanArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::System::IntPtr,
                1usize,
            >("NewBooleanArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewBooleanArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewByteArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::IntPtr, 1usize>("NewByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewByteArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewCharArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::IntPtr, 1usize>("NewCharArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewCharArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewDirectByteBufferFromNativeArray<T>(
        buffer: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Collections::NativeArray_1<T>),
                crate::System::IntPtr,
                1usize,
            >("NewDirectByteBufferFromNativeArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewDirectByteBufferFromNativeArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (buffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewDirectByteBuffer_Il2CppObject_i64_0(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        capacity: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i64),
                crate::System::IntPtr,
                2usize,
            >("NewDirectByteBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewDirectByteBuffer", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (buffer, capacity))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewDirectByteBuffer_NativeArray_1_1(
        buffer: crate::Unity::Collections::NativeArray_1<u8>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Collections::NativeArray_1<u8>),
                crate::System::IntPtr,
                1usize,
            >("NewDirectByteBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewDirectByteBuffer", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (buffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewDirectByteBuffer_NativeArray_1_2(
        buffer: crate::Unity::Collections::NativeArray_1<i8>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Collections::NativeArray_1<i8>),
                crate::System::IntPtr,
                1usize,
            >("NewDirectByteBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewDirectByteBuffer", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (buffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewDoubleArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::IntPtr, 1usize>("NewDoubleArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewDoubleArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewFloatArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::IntPtr, 1usize>("NewFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewFloatArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("NewGlobalRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewGlobalRef", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewIntArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::IntPtr, 1usize>("NewIntArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewIntArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewLocalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("NewLocalRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewLocalRef", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewLongArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::IntPtr, 1usize>("NewLongArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewLongArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewObjectA(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                crate::System::IntPtr,
                3usize,
            >("NewObjectA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewObjectA", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewObjectArray(
        _cordl_size: i32,
        clazz: crate::System::IntPtr,
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::System::IntPtr, crate::System::IntPtr),
                crate::System::IntPtr,
                3usize,
            >("NewObjectArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewObjectArray", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size, clazz, obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewObject_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
                    >,
                ),
                crate::System::IntPtr,
                3usize,
            >("NewObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewObject", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewObject_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::Span_1<crate::UnityEngine::jvalue>,
                ),
                crate::System::IntPtr,
                3usize,
            >("NewObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewObject", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodID, args))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewSByteArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::IntPtr, 1usize>("NewSByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewSByteArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewShortArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::IntPtr, 1usize>("NewShortArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewShortArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewStringFromStr(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::IntPtr,
                1usize,
            >("NewStringFromStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewStringFromStr", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (chars))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewStringUTF(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::IntPtr,
                1usize,
            >("NewStringUTF")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewStringUTF", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (bytes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewString_Il2CppArray1(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>),
                crate::System::IntPtr,
                1usize,
            >("NewString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewString", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (chars))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewString_Il2CppString0(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::IntPtr,
                1usize,
            >("NewString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewString", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (chars))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewWeakGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("NewWeakGlobalRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewWeakGlobalRef", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopLocalFrame(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("PopLocalFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PopLocalFrame", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PushLocalFrame(capacity: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("PushLocalFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PushLocalFrame", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (capacity)) };
        Ok(__cordl_ret.into())
    }
    pub fn QueueDeleteGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("QueueDeleteGlobalRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "QueueDeleteGlobalRef", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNatives(
        clazz: crate::System::IntPtr,
        methods: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::JNINativeMethod>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::JNINativeMethod,
                        >,
                    >,
                ),
                i32,
                2usize,
            >("RegisterNatives")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterNatives", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (clazz, methods)) };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNativesAllocate(
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::System::IntPtr,
                1usize,
            >("RegisterNativesAllocate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterNativesAllocate", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNativesAndFree(
        clazz: crate::System::IntPtr,
        natives: crate::System::IntPtr,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i32),
                i32,
                3usize,
            >("RegisterNativesAndFree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterNativesAndFree", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (clazz, natives, n))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNativesSet(
        natives: crate::System::IntPtr,
        idx: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fnPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("RegisterNativesSet")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterNativesSet", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (natives, idx, name, signature, fnPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBooleanArrayElement__cordl_bool1(
        array: crate::System::IntPtr,
        index: i32,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetBooleanArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetBooleanArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBooleanArrayElement_u8_0(
        array: crate::System::IntPtr,
        index: i32,
        val: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, u8),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetBooleanArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetBooleanArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBooleanField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetBooleanField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetBooleanField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetByteArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, i8),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetByteArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetByteArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, u8),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetByteField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetByteField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCharArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, char),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetCharArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetCharArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCharField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, char),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetCharField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetCharField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDoubleArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, f64),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetDoubleArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDoubleArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDoubleField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, f64),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetDoubleField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDoubleField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFloatArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetFloatArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetFloatArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFloatField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetFloatField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetFloatField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetIntArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetIntArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetIntArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetIntField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetIntField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetIntField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLongArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, i64),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetLongArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetLongArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLongField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i64),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetLongField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetLongField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetObjectArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetObjectArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetObjectField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetObjectField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSByteArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, i8),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetSByteArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetSByteArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i8),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetSByteField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetSByteField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetShortArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, i16),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetShortArrayElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetShortArrayElement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (array, index, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetShortField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i16),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetShortField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetShortField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticBooleanField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticBooleanField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticBooleanField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticByteField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, u8),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticByteField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticByteField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticCharField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, char),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticCharField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticCharField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticDoubleField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, f64),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticDoubleField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticDoubleField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticFloatField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticFloatField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticFloatField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticIntField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticIntField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticIntField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticLongField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i64),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticLongField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticLongField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticObjectField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticObjectField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticObjectField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticSByteField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i8),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticSByteField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticSByteField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticShortField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, i16),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticShortField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticShortField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticStringField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStaticStringField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStaticStringField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStringField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetStringField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStringField", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, fieldID, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Throw(obj: crate::System::IntPtr) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::IntPtr), i32, 1usize>("Throw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Throw", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowNew(
        clazz: crate::System::IntPtr,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                i32,
                2usize,
            >("ThrowNew")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowNew", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (clazz, message)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBooleanArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>),
                crate::System::IntPtr,
                1usize,
            >("ToBooleanArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBooleanArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToByteArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                crate::System::IntPtr,
                1usize,
            >("ToByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByteArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToCharArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>),
                crate::System::IntPtr,
                1usize,
            >("ToCharArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToCharArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToCharArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                crate::System::IntPtr,
                2usize,
            >("ToCharArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToCharArray", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDoubleArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>),
                crate::System::IntPtr,
                1usize,
            >("ToDoubleArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDoubleArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDoubleArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                crate::System::IntPtr,
                2usize,
            >("ToDoubleArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDoubleArray", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFloatArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>),
                crate::System::IntPtr,
                1usize,
            >("ToFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToFloatArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFloatArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                crate::System::IntPtr,
                2usize,
            >("ToFloatArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToFloatArray", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToIntArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>),
                crate::System::IntPtr,
                1usize,
            >("ToIntArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToIntArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToIntArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                crate::System::IntPtr,
                2usize,
            >("ToIntArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToIntArray", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToLongArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>),
                crate::System::IntPtr,
                1usize,
            >("ToLongArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToLongArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToLongArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                crate::System::IntPtr,
                2usize,
            >("ToLongArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToLongArray", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToObjectArray_Il2CppArray2(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
                >),
                crate::System::IntPtr,
                1usize,
            >("ToObjectArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToObjectArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToObjectArray_Il2CppArray_IntPtr1(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
        arrayClass: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
                    >,
                    crate::System::IntPtr,
                ),
                crate::System::IntPtr,
                2usize,
            >("ToObjectArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToObjectArray", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, arrayClass))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToObjectArray_Il2CppObject_i32_IntPtr0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
        arrayClass: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    crate::System::IntPtr,
                ),
                crate::System::IntPtr,
                3usize,
            >("ToObjectArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToObjectArray", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, length, arrayClass))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToReflectedField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, bool),
                crate::System::IntPtr,
                3usize,
            >("ToReflectedField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToReflectedField", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, fieldID, isStatic))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToReflectedMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, bool),
                crate::System::IntPtr,
                3usize,
            >("ToReflectedMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToReflectedMethod", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodID, isStatic))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByteArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>),
                crate::System::IntPtr,
                1usize,
            >("ToSByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByteArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByteArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                crate::System::IntPtr,
                2usize,
            >("ToSByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByteArray", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToShortArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>),
                crate::System::IntPtr,
                1usize,
            >("ToShortArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToShortArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToShortArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                crate::System::IntPtr,
                2usize,
            >("ToShortArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToShortArray", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (array, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterNatives(
        clazz: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("UnregisterNatives")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnregisterNatives", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (clazz)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJNI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
