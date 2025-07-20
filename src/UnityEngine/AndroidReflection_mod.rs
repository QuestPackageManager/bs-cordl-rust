#[cfg(feature = "UnityEngine+AndroidReflection")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidReflection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AndroidReflection")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AndroidReflection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AndroidReflection";
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
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl std::ops::Deref for crate::UnityEngine::AndroidReflection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidReflection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl crate::UnityEngine::AndroidReflection {
    pub fn GetConstructorMember(
        jclass: crate::System::IntPtr,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::System::IntPtr,
                2usize,
            >("GetConstructorMember")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "GetConstructorMember", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (jclass, signature))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldClass(
        field: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("GetFieldClass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "GetFieldClass", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (field))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldMember(
        jclass: crate::System::IntPtr,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                crate::System::IntPtr,
                4usize,
            >("GetFieldMember")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "GetFieldMember", 4usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (jclass, fieldName, signature, isStatic))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldSignature(
        field: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetFieldSignature")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "GetFieldSignature", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (field))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID(
        clazz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::System::IntPtr,
                3usize,
            >("GetMethodID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "GetMethodID", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodName, signature))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodMember(
        jclass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                crate::System::IntPtr,
                4usize,
            >("GetMethodMember")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "GetMethodMember", 4usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (jclass, methodName, signature, isStatic))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticMethodID(
        clazz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::System::IntPtr,
                3usize,
            >("GetStaticMethodID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "GetStaticMethodID", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (clazz, methodName, signature))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAssignableFrom(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        from: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                bool,
                2usize,
            >("IsAssignableFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "IsAssignableFrom", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (t, from))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsPrimitive(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("IsPrimitive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "IsPrimitive", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn NewProxyInstance(
        player: crate::System::IntPtr,
        delegateHandle: crate::System::IntPtr,
        interfaze: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, crate::System::IntPtr),
                crate::System::IntPtr,
                3usize,
            >("NewProxyInstance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "NewProxyInstance", 3usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (player, delegateHandle, interfaze))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNativeExceptionOnProxy(
        proxy: crate::System::IntPtr,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        methodNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<crate::System::Exception>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetNativeExceptionOnProxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::AndroidReflection as quest_hook::libil2cpp::Type
                    > ::class(), "SetNativeExceptionOnProxy", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (proxy, e, methodNotFound))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidReflection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
