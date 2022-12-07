use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use sha2::{Digest, Sha256};

fn run_guest(num_iter: u32, method_id: &[u8], method_path: &str) -> Vec<u32> {
    let image = std::fs::read(method_path).expect("image");
    let mut prover = Prover::new(&image, method_id).expect("prover");

    let mut guest_input = Vec::from([0u32; 9]);
    guest_input[0] = num_iter;
    prover
        .add_input(to_vec(&guest_input).as_ref().expect("guest input"))
        .expect("prover input");

    let receipt = prover.run().expect("receipt");

    from_slice(&receipt.get_journal_vec().expect("journal")).expect("result")
}

fn main() {
    for num_iter in 1..3 {
        println!("num_iter = {}", num_iter);

        let host_output: Vec<u8> = {
            let mut host_output = Vec::from([0u8; 32]);

            for _i in 0..num_iter {
                let mut hasher = Sha256::new();
                hasher.update(&host_output);
                host_output = hasher.finalize().to_vec();
            }

            host_output
        };

        {
            println!("iter_sha2_bytes");
            let journal = run_guest(num_iter, methods::ITER_SHA2_BYTES_ID, methods::ITER_SHA2_BYTES_PATH);
            let guest_output: Vec<u8> = journal.iter().map(|x| x.to_be_bytes()).flatten().collect();
            assert_eq!(host_output, guest_output);
        };

        {
            println!("iter_sha2_words");
            let journal = run_guest(num_iter, methods::ITER_SHA2_WORDS_ID, methods::ITER_SHA2_WORDS_PATH);
            let guest_output: Vec<u8> = journal.iter().map(|x| x.to_be_bytes()).flatten().collect();
            assert_eq!(host_output, guest_output);
        };
    }

    println!("Done");
}
