use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    pub struct InputValues {
        v1: u8,
        v2: u8,
    }

    #[instruction]
    pub fn add_together(input_ctxt: Enc<Shared, InputValues>) -> Enc<Shared, u16> {
        let input = input_ctxt.to_arcis();
        let sum = input.v1 as u16 + input.v2 as u16;
        input_ctxt.owner.from_arcis(sum)
    }

    #[derive(Copy, Clone)]
    pub struct CounterState {
        counter: u64,
    }

    /// Initialize a new counter with value 0, encrypted for the MXE only.
    /// The state is stored on-chain and only the MXE can decrypt it.
    #[instruction]
    pub fn init_counter(mxe: Mxe) -> Enc<Mxe, CounterState> {
        let counter = CounterState { counter: 0 };
        mxe.from_arcis(counter)
    }

    /// Increment the counter. Takes MXE-encrypted state by reference,
    /// returns updated MXE-encrypted state.
    #[instruction]
    pub fn increment_counter(
        counter_ctxt: Enc<Mxe, &CounterState>,
    ) -> Enc<Mxe, CounterState> {
        let mut counter = *(counter_ctxt.to_arcis());
        counter.counter += 1;
        counter_ctxt.owner.from_arcis(counter)
    }

    /// Read the counter value and re-encrypt it for a specific user.
    /// This allows sharing the MXE-encrypted state with the outside world.
    #[instruction]
    pub fn get_counter(
        counter_ctxt: Enc<Mxe, &CounterState>,
        recipient: Shared,
    ) -> Enc<Shared, CounterState> {
        let counter = *(counter_ctxt.to_arcis());
        // Re-encrypt for the recipient so they can decrypt it
        recipient.from_arcis(counter)
    }
}
