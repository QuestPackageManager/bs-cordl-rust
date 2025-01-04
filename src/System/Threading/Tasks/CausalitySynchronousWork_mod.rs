#[cfg(feature = "System+Threading+Tasks+CausalitySynchronousWork")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CausalitySynchronousWork {
    #[default]
    CompletionNotification = 0i32,
    Execution = 2i32,
    ProgressNotification = 1i32,
}
#[cfg(feature = "System+Threading+Tasks+CausalitySynchronousWork")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::CausalitySynchronousWork => "System.Threading.Tasks"
    ."CausalitySynchronousWork"
);
