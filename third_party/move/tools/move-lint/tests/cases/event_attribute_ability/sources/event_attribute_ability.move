module NamedAddr::Detector {
    #[event]
    struct TransferEvent has drop, store {
        sender: address,
        receiver: address,
        amount: u64
    }

    #[event]
    struct InvalidTransferEvent has drop {
        sender: address,
        receiver: address,
        amount: u64
    }
}
