#[cfg(feature = "GoodCutScoringElement")]
#[repr(C)]
#[derive(Debug)]
pub struct GoodCutScoringElement {
    __cordl_parent: crate::GlobalNamespace::ScoringElement,
    pub _cutScoreBuffer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CutScoreBuffer,
    >,
    pub _multiplierEventType: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    pub _wouldBeCorrectCutBestPossibleMultiplierEventType: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
}
#[cfg(feature = "GoodCutScoringElement")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GoodCutScoringElement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GoodCutScoringElement";
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
#[cfg(feature = "GoodCutScoringElement")]
impl std::ops::Deref for crate::GlobalNamespace::GoodCutScoringElement {
    type Target = crate::GlobalNamespace::ScoringElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl std::ops::DerefMut for crate::GlobalNamespace::GoodCutScoringElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl crate::GlobalNamespace::GoodCutScoringElement {
    #[cfg(feature = "GoodCutScoringElement+Pool")]
    pub type Pool = crate::GlobalNamespace::GoodCutScoringElement_Pool;
    pub fn HandleCutScoreBufferDidFinish(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CutScoreBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CutScoreBuffer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleCutScoreBufferDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleCutScoreBufferDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cutScoreBuffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteCutInfo),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (noteCutInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reinitialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reinitialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Reinitialize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_cutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_cutScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_cutScore", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_cutScoreBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyCutScoreBuffer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IReadonlyCutScoreBuffer,
                >,
                0usize,
            >("get_cutScoreBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_cutScoreBuffer", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyCutScoreBuffer,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_executionOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_executionOrder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_executionOrder", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplierEventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
                0usize,
            >("get_multiplierEventType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_multiplierEventType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_wouldBeCorrectCutBestPossibleMultiplierEventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
                0usize,
            >("get_wouldBeCorrectCutBestPossibleMultiplierEventType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_wouldBeCorrectCutBestPossibleMultiplierEventType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GoodCutScoringElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl AsRef<crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver>
for crate::GlobalNamespace::GoodCutScoringElement {
    fn as_ref(&self) -> &crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl AsMut<crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver>
for crate::GlobalNamespace::GoodCutScoringElement {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct GoodCutScoringElement_Pool {
    __cordl_parent: crate::GlobalNamespace::ScoringElement_Pool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GoodCutScoringElement>,
    >,
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GoodCutScoringElement_Pool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GoodCutScoringElement/Pool";
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
#[cfg(feature = "GoodCutScoringElement+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::GoodCutScoringElement_Pool {
    type Target = crate::GlobalNamespace::ScoringElement_Pool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GoodCutScoringElement>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::GoodCutScoringElement_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
impl crate::GlobalNamespace::GoodCutScoringElement_Pool {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GoodCutScoringElement_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
