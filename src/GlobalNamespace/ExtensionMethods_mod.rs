#[cfg(feature = "ExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ExtensionMethods")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ExtensionMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ExtensionMethods";
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
#[cfg(feature = "ExtensionMethods")]
impl std::ops::Deref for crate::GlobalNamespace::ExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ExtensionMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::ExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ExtensionMethods")]
impl crate::GlobalNamespace::ExtensionMethods {
    pub fn ContainsLayer(
        layerMask: crate::UnityEngine::LayerMask,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::LayerMask, i32),
                bool,
                2usize,
            >("ContainsLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "ContainsLayer", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (layerMask, layer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTexture2D(
        renderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        textureFormat: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    crate::UnityEngine::TextureFormat,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                2usize,
            >("CreateTexture2D")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "CreateTexture2D", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method.invoke_unchecked((), (renderTexture, textureFormat))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPath(
        current: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "GetPath", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (current))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRange<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IReadOnlyList_1<T>,
                    >,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<T>,
                >,
                3usize,
            >("GetRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "GetRange", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = unsafe { method.invoke_unchecked((), (list, index, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsDescendantOf(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                ),
                bool,
                2usize,
            >("IsDescendantOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "IsDescendantOf", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (transform, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastUnsafe<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<T>,
                >),
                T,
                1usize,
            >("LastUnsafe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "LastUnsafe", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn Reflect(
        source: crate::UnityEngine::Quaternion,
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Quaternion, crate::UnityEngine::Vector3),
                crate::UnityEngine::Quaternion,
                2usize,
            >("Reflect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "Reflect", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            method.invoke_unchecked((), (source, normal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Rotate(
        vector: crate::UnityEngine::Vector2,
        rads: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2, f32),
                crate::UnityEngine::Vector2,
                2usize,
            >("Rotate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "Rotate", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (vector, rads))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPositionAndRotation(
        tr: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        pos: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Quaternion,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetLocalPositionAndRotation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(),
                    "SetLocalPositionAndRotation", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tr, pos, rot))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSeed(
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
        seed: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>, u32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetSeed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "SetSeed", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (particleSystem, seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartUniqueCoroutine_Func_1_0(
        m: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::IEnumerator,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
                2usize,
            >("StartUniqueCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "StartUniqueCoroutine",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine> = unsafe {
            method.invoke_unchecked((), (m, func))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartUniqueCoroutine_Func_2_T1<T>(
        m: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                T,
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
        value: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            T,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::IEnumerator,
                            >,
                        >,
                    >,
                    T,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
                3usize,
            >("StartUniqueCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "StartUniqueCoroutine",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine> = unsafe {
            method.invoke_unchecked((), (m, func, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StopUniqueCoroutine_Func_1_0(
        m: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::IEnumerator,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("StopUniqueCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "StopUniqueCoroutine",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (m, func))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StopUniqueCoroutine_Func_2_1<T>(
        m: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                T,
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ExtensionMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            T,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::IEnumerator,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("StopUniqueCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ExtensionMethods as
                    quest_hook::libil2cpp::Type > ::class(), "StopUniqueCoroutine",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (m, func))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
