use crate::define_options;
use std::ffi::c_ulong;

define_options!(pub Flags(c_ulong));

/// #[doc(alias = "dispatch_block_flags")]

/// Flags to pass to the dispatch_block_create* functions.
impl Flags {
    /// Flag indicating that a dispatch block object should act as a barrier block
    /// when submitted to a DISPATCH_QUEUE_CONCURRENT queue.
    /// See dispatch_barrier_async() for details.
    /// This flag has no effect when the dispatch block object is invoked directly.
    #[doc(alias = "DISPATCH_BLOCK_BARRIER")]
    pub const BARRIER: Self = Flags(0x1);

    /// Flag indicating that a dispatch block object should execute disassociated
    /// from current execution context attributes such as os_activity_t
    /// and properties of the current IPC request (if any). With regard to QoS class,
    /// the behavior is the same as for DISPATCH_BLOCK_NO_QOS. If invoked directly,
    /// the block object will remove the other attributes from the calling thread for
    /// the duration of the block body (before applying attributes assigned to the
    /// block object, if any). If submitted to a queue, the block object will be
    /// executed with the attributes of the queue (or any attributes specifically
    /// assigned to the block object).
    #[doc(alias = "DISPATCH_BLOCK_DETACHED")]
    pub const DETACHED: Self = Flags(0x2);

    /// Flag indicating that a dispatch block object should be assigned the execution
    /// context attributes that are current at the time the block object is created.
    /// This applies to attributes such as QOS class, os_activity_t and properties of
    /// the current IPC request (if any). If invoked directly, the block object will
    /// apply these attributes to the calling thread for the duration of the block
    /// body. If the block object is submitted to a queue, this flag replaces the
    /// default behavior of associating the submitted block instance with the
    /// execution context attributes that are current at the time of submission.
    /// If a specific QOS class is assigned with DISPATCH_BLOCK_NO_QOS_CLASS or
    /// dispatch_block_create_with_qos_class(), that QOS class takes precedence over
    /// the QOS class assignment indicated by this flag.
    #[doc(alias = "DISPATCH_BLOCK_ASSIGN_CURRENT")]
    pub const ASSIGN_CURRENT: Self = Flags(0x4);

    /// Flag indicating that a dispatch block object should be not be assigned a QOS
    /// class. If invoked directly, the block object will be executed with the QOS
    /// class of the calling thread. If the block object is submitted to a queue,
    /// this replaces the default behavior of associating the submitted block
    /// instance with the QOS class current at the time of submission.
    /// This flag is ignored if a specific QOS class is assigned with
    /// dispatch_block_create_with_qos_class().
    #[doc(alias = "DISPATCH_BLOCK_NO_QOS_CLASS")]
    pub const NO_QOS_CLASS: Self = Flags(0x8);

    /// Flag indicating that execution of a dispatch block object submitted to a
    /// queue should prefer the QOS class assigned to the queue over the QOS class
    /// assigned to the block (resp. associated with the block at the time of
    /// submission). The latter will only be used if the queue in question does not
    /// have an assigned QOS class, as long as doing so does not result in a QOS
    /// class lower than the QOS class inherited from the queue's target queue.
    /// This flag is the default when a dispatch block object is submitted to a queue
    /// for asynchronous execution and has no effect when the dispatch block object
    /// is invoked directly. It is ignored if DISPATCH_BLOCK_ENFORCE_QOS_CLASS is
    /// also passed.
    #[doc(alias = "DISPATCH_BLOCK_INHERIT_QOS_CLASS")]
    pub const INHERIT_QOS_CLASS: Self = Flags(0x10);

    /// Flag indicating that execution of a dispatch block object submitted to a
    /// queue should prefer the QOS class assigned to the block (resp. associated
    /// with the block at the time of submission) over the QOS class assigned to the
    /// queue, as long as doing so will not result in a lower QOS class.
    /// This flag is the default when a dispatch block object is submitted to a queue
    /// for synchronous execution or when the dispatch block object is invoked
    /// directly.
    #[doc(alias = "DISPATCH_BLOCK_ENFORCE_QOS_CLASS")]
    pub const ENFORCE_QOS_CLASS: Self = Flags(0x20);
}
