#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation")]
#[repr(C)]
#[derive(Debug)]
pub struct CurveAnimation {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _bezierCurve: quest_hook::libil2cpp::Gc<
        crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve,
    >,
    pub _duration: f32,
    pub _delay: f32,
    pub _speedCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub _playOnAwake: bool,
    pub onStart: quest_hook::libil2cpp::Gc<
        crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent,
    >,
    pub afterDelay: quest_hook::libil2cpp::Gc<
        crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent,
    >,
    pub onIterate: quest_hook::libil2cpp::Gc<
        crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent_1<
            crate::UnityEngine::Vector3,
        >,
    >,
    pub onFinish: quest_hook::libil2cpp::Gc<
        crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent,
    >,
    pub _isPlaying_k__BackingField: bool,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension.BezierCurves";
    const CLASS_NAME: &'static str = "CurveAnimation";
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
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation")]
impl std::ops::Deref for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation")]
impl crate::BGLib::UnityExtension::BezierCurves::CurveAnimation {
    #[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent")]
    pub type CurveEvent = crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent;
    #[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent_1")]
    pub type CurveEvent_1<T: quest_hook::libil2cpp::Type> = crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent_1<
        T,
    >;
    pub fn Animate__cordl_bool0(
        &mut self,
        withDelay: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("Animate", (withDelay))?;
        Ok(__cordl_ret.into())
    }
    pub fn Animate_f32_1(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Animate", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StartAnimation(
        &mut self,
        withDelay: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartAnimation", (withDelay))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPlaying(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPlaying", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isPlaying(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isPlaying", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct CurveAnimation_CurveEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension.BezierCurves";
    const CLASS_NAME: &'static str = "CurveEvent";
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
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent")]
impl std::ops::Deref
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent {
    type Target = crate::UnityEngine::Events::UnityEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent")]
impl std::ops::DerefMut
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent")]
impl crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent_1")]
#[repr(C)]
#[derive(Debug)]
pub struct CurveAnimation_CurveEvent_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension.BezierCurves";
    const CLASS_NAME: &'static str = "CurveEvent`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "BGLib.UnityExtension.BezierCurves",
                        "CurveEvent`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent_1<T> {
    type Target = crate::UnityEngine::Events::UnityEvent_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveAnimation+CurveEvent_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::BezierCurves::CurveAnimation_CurveEvent_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
