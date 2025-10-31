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

    pub struct SwapInput {
        amount_in: u64,
        // rate in parts-per-million (PPM). amount_out = amount_in * rate_ppm / 1_000_000
        rate_ppm: u64,
    }

    #[instruction]
    pub fn swap(input_ctxt: Enc<Shared, SwapInput>) -> Enc<Shared, u64> {
        let input = input_ctxt.to_arcis();
        let amount_out = ((input.amount_in as u128) * (input.rate_ppm as u128) / 1_000_000u128) as u64;
        input_ctxt.owner.from_arcis(amount_out)
    }
}
