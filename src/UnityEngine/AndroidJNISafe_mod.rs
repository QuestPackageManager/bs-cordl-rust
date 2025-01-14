#[cfg(feature = "UnityEngine+AndroidJNISafe")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJNISafe {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AndroidJNISafe {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AndroidJNISafe";
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
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl std::ops::Deref for crate::UnityEngine::AndroidJNISafe {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJNISafe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl crate::UnityEngine::AndroidJNISafe {
    pub fn CallBooleanMethod(
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
    pub fn CallCharMethod(
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
    pub fn CallDoubleMethod(
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
    pub fn CallFloatMethod(
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
    pub fn CallIntMethod(
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
    pub fn CallLongMethod(
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
    pub fn CallSByteMethod(
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
    pub fn CallShortMethod(
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
    pub fn CallStaticBooleanMethod(
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
    pub fn CallStaticCharMethod(
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
    pub fn CallStaticDoubleMethod(
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
    pub fn CallStaticFloatMethod(
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
    pub fn CallStaticIntMethod(
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
    pub fn CallStaticLongMethod(
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
    pub fn CallStaticSByteMethod(
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
    pub fn CallStaticShortMethod(
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
    pub fn CallStringMethod(
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
    pub fn CallVoidMethod(
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
    pub fn CheckException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("CheckException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteLocalRef(
        localref: crate::System::IntPtr,
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
            method.invoke_unchecked((), (localref))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteWeakGlobalRef(
        globalref: crate::System::IntPtr,
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
            method.invoke_unchecked((), (globalref))
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
        obj: crate::System::IntPtr,
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
            method.invoke_unchecked((), (obj, name, sig))
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
        ptr: crate::System::IntPtr,
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
            method.invoke_unchecked((), (ptr))
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
    pub fn NewObject(
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
    pub fn NewString(
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
    pub fn QueueDeleteGlobalRef(
        globalref: crate::System::IntPtr,
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
            method.invoke_unchecked((), (globalref))
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
    pub fn ToCharArray(
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
    pub fn ToDoubleArray(
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
    pub fn ToFloatArray(
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
    pub fn ToIntArray(
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
    pub fn ToLongArray(
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
    pub fn ToObjectArray(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
        _cordl_type: crate::System::IntPtr,
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
            method.invoke_unchecked((), (array, _cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByteArray(
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
    pub fn ToShortArray(
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
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJNISafe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
