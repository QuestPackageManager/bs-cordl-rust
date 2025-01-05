#[cfg(feature = "UnityEngine+AndroidReflection")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidReflection {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+AndroidReflection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidReflection => "UnityEngine"
    ."AndroidReflection"
);
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl std::ops::Deref for crate::UnityEngine::AndroidReflection {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConstructorMember", (jclass, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldClass(
        field: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldClass", (field))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldMember(
        jclass: crate::System::IntPtr,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldMember", (jclass, fieldName, signature, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldSignature(
        field: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldSignature", (field))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID(
        clazz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (clazz, methodName, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodMember(
        jclass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodMember", (jclass, methodName, signature, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticMethodID(
        clazz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticMethodID", (clazz, methodName, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAssignableFrom(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        from: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAssignableFrom", (t, from))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrimitive(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrimitive", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewProxyInstance(
        player: crate::System::IntPtr,
        delegateHandle: crate::System::IntPtr,
        interfaze: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewProxyInstance", (player, delegateHandle, interfaze))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNativeExceptionOnProxy(
        proxy: crate::System::IntPtr,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        methodNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetNativeExceptionOnProxy", (proxy, e, methodNotFound))?;
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
