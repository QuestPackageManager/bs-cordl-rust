#[cfg(feature = "cordl_class_UnityEngine+Rendering+AtlasAllocator")]
#[repr(C)]
#[derive(Debug)]
pub struct AtlasAllocator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Root: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode>,
    pub m_Width: i32,
    pub m_Height: i32,
    pub powerOfTwoPadding: bool,
    pub m_NodePool: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ObjectPool_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode>,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AtlasAllocator")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::AtlasAllocator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "AtlasAllocator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+AtlasAllocator")]
impl std::ops::Deref for crate::UnityEngine::Rendering::AtlasAllocator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+AtlasAllocator")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::AtlasAllocator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+AtlasAllocator")]
impl crate::UnityEngine::Rendering::AtlasAllocator {
    #[cfg(feature = "UnityEngine+Rendering+AtlasAllocator+AtlasNode")]
    pub type AtlasNode = crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode;
    pub fn Allocate(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                        i32,
                        i32,
                    ), bool, 3usize>("Allocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Allocate",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (result, width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        width: i32,
        height: i32,
        potPadding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, potPadding))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        width: i32,
        height: i32,
        potPadding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, bool), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (width, height, potPadding))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AtlasAllocator")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::AtlasAllocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AtlasAllocator+AtlasNode")]
#[repr(C)]
#[derive(Debug)]
pub struct AtlasAllocator_AtlasNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_RightChild:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode>,
    pub m_BottomChild:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode>,
    pub m_Rect: crate::UnityEngine::Vector4,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AtlasAllocator+AtlasNode")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "AtlasAllocator/AtlasNode";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+AtlasAllocator+AtlasNode")]
impl std::ops::Deref for crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+AtlasAllocator+AtlasNode")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+AtlasAllocator+AtlasNode")]
impl crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode {
    pub fn Allocate(
        &mut self,
        pool: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::ObjectPool_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode,
                    >,
                >,
            >,
        >,
        width: i32,
        height: i32,
        powerOfTwoPadding: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ObjectPool_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode,
                                    >,
                                >,
                            >,
                        >,
                        i32,
                        i32,
                        bool,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode,
                    >, 4usize>("Allocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Allocate",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (pool, width, height, powerOfTwoPadding))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
        pool: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::ObjectPool_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ObjectPool_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode,
                                >,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("Release")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Release",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pool))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+AtlasAllocator+AtlasNode")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::AtlasAllocator_AtlasNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
